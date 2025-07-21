
use x25519_dalek::{EphemeralSecret, StaticSecret, PublicKey, SharedSecret};
use aes_gcm::aead::{Aead, AeadCore, KeyInit};
use rand::{rngs::OsRng, RngCore};
use aes_gcm::{Aes256Gcm, Nonce, Key};
use crate::error::Error;
use rand::Rng;
use rand::RngCore;
use hkdf::Hkdf;
use sha2::Sha256;
use log::trace;
use super::{forge, UserKey};

pub struct Postman {
    user_key: UserKey,
    public_key: PublicKey,
    nonce: [u8; 32],
    iv: [u8; 12]
}


impl Postman {

    pub fn new(user_key: &UserKey, public_key: &PublicKey) -> Self {
        Self {
            user_key: user_key.clone(),
            public_key: public_key.clone(),
            nonce: [0; 32],
            iv: [0; 12]
        }
    }

    /// Encrypt a message to recipient
    pub fn encrypt(&self, message: &[u8]) -> Vec<u8>{

        // Generate encrypt key and iv
        let mut rng = rand::thread_rng();
        let encryption_key: [u8; 32] = rng.gen();
        let iv: [u8; 12] = rng.gen();
 
        // Encrypt the message
        let key: &Key<Aes256Gcm> = &encryption_key.into();
        let cipher = Aes256Gcm::new(&key);
        let ciphertext = cipher.encrypt(&iv.into(), message.as_ref()).unwrap();

        // Concatenate the encryption key and IV
        let outer_seal = [&encryption_key[..], &iv[..]].concat();
        let outer_iv: [u8; 12] = rng.gen();
        let (child_key, nonce): ([u8; 32], [u8; 32]) = self.derive_child(None);

        // Encrypt seal
        let seal_key: &Key<Aes256Gcm> = &child_key.into();
        let seal_cipher = Aes256Gcm::new(&seal_key);
        let encrypted_outer_seal = seal_cipher.encrypt(&outer_iv.into(), outer_seal.as_ref()).unwrap();

        // Generate the concatenated header
        let mut header: Vec<u8> = Vec::new();
        header.push(forge::PREFIX);
        header.push(forge::VERSION);
        header.extend_from_slice(&encrypted_outer_seal);
        header.extend_from_slice(&outer_iv);
        header.extend_from_slice(&nonce);

        // Prepend the header to the encrypted message
        [header, ciphertext].concat()
    }

    /// Decrypt message
    pub fn decrypt(&self, payload: &[u8]) -> Result<Vec<u8>, Error> {

        // Check header
        if payload[0] != forge::PREFIX {
            return Err(Error::Generic("Invalid message prefix.".to_string()));
        } else if payload[1] != forge::VERSION {
            return Err(Error::Generic("Invalid message version.".to_string()));
        }

        // Get outer iv and nonce
        let mut outer_iv: [u8; 12] = [0; 12];
        let mut nonce: [u8; 32] = [0; 32];
        outer_iv.copy_from_slice(&payload[62..74]);
        nonce.copy_from_slice(&payload[74..106]);

        // Derive child 
        let (child_key, _): ([u8; 32], [u8; 32]) = self.derive_child(Some(nonce));
        let seal_key = Key::<Aes256Gcm>::from_slice(&child_key);

        // Decrypd seal
        let seal_cipher = Aes256Gcm::new(&seal_key);
        let inner_seal = match seal_cipher.decrypt(&outer_iv.into(), payload[2..62].as_ref()) {
            Ok(r) => r,
            Err(e) => return Err( Error::Generic("Invalid encryption password.".to_string())) 
        };

        // Get iv and encryption key
        let mut iv: [u8; 12] = [0; 12];
        iv.copy_from_slice(&inner_seal[32..44]);
        let msg_key = Key::<Aes256Gcm>::from_slice(&inner_seal[0..32]);

        // Decrypt message
        let msg_cipher = Aes256Gcm::new(&msg_key);
        let message = match msg_cipher.decrypt(&iv.into(), payload[106..].as_ref()) {
            Ok(r) => r,
            Err(e) => return Err( Error::Generic("Invalid encryption password.".to_string())) 
        };

        Ok(message)
    }

    /// Derive child from shard secret
    fn derive_child(&self, previous_nonce: Option<[u8; 32]>) -> ([u8; 32], [u8; 32]) {

        // Get nonce
        let nonce = forge::get_nonce(previous_nonce);
        let shared_secret = self.user_key.secret.diffie_hellman(&self.public_key);

        // Derive child key from nonce
        let mut child_bytes = [0u8; 32];
        Hkdf::<Sha256>::from_prk(&shared_secret.to_bytes())
            .expect("Unable to derive child key")
            .expand(&nonce, &mut child_bytes)
            .unwrap();



        (child_bytes, nonce)
    }





}





