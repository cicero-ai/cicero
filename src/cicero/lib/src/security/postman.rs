
use x25519_dalek::{EphemeralSecret, StaticSecret, PublicKey, SharedSecret};
use aes_gcm::aead::{Aead, AeadCore, KeyInit, OsRng};
use aes_gcm::aead::rand_core::RngCore;
use aes_gcm::{Aes256Gcm, Nonce, Key};
use crate::error::Error;
use rand::Rng;
use hkdf::Hkdf;
use sha2::Sha256;
use log::trace;
use super::{forge, UserKey};

/// A utility for encrypting and decrypting messages in two-way communication.
/// Uses X25519 for key exchange and AES-256-GCM for encryption.
pub struct Postman {
    user_key: UserKey,
    public_key: PublicKey,
}

impl Postman {
    pub fn new(user_key: &UserKey, public_key: &PublicKey) -> Self {
        Self {
            user_key: user_key.clone(),
            public_key: public_key.clone(),
        }
    }

    /// Encrypts a message for the recipient using AES-256-GCM.
    /// Output: [prefix | version | encrypted_key+iv | outer_iv | nonce | ciphertext].
    pub fn encrypt(&self, message: &[u8]) -> Result<Vec<u8>, Error> {
        let mut rng = OsRng;

    // Generate encryption key
        let mut encryption_key = [0u8; 32];
        rng.fill_bytes(&mut encryption_key);

        // Generate iv
        let mut iv = [0u8; 12];
        rng.fill_bytes(&mut iv);

        let key: &Key<Aes256Gcm> = &encryption_key.into();
        let cipher = Aes256Gcm::new(&key);
        let ciphertext = cipher.encrypt(&iv.into(), message.as_ref())
            .map_err(|e| Error::AES(e.to_string()) )?;

        let outer_seal = [&encryption_key[..], &iv[..]].concat();
        let mut outer_iv = [0u8; 12];
        rng.fill_bytes(&mut outer_iv);

        let (child_key, nonce) = self.derive_session_key(None);
        let seal_cipher = Aes256Gcm::new_from_slice(&child_key)
            .map_err(|e| Error::AES(e.to_string()) )?;

        // Encrypt seal
        let seal_key: &Key<Aes256Gcm> = &child_key.into();
        let seal_cipher = Aes256Gcm::new(&seal_key);
        let encrypted_outer_seal = seal_cipher.encrypt(&outer_iv.into(), outer_seal.as_ref())
            .map_err(|e| Error::AES(e.to_string()) )?;

        let mut header = vec![forge::PREFIX, forge::VERSION];
        header.extend_from_slice(&encrypted_outer_seal);
        header.extend_from_slice(&outer_iv);
        header.extend_from_slice(&nonce);

        Ok([header, ciphertext].concat())
    }

    /// Decrypts a message encrypted with `encrypt`.
    /// Returns the plaintext or an error if the keys or payload are invalid.
    pub fn decrypt(&self, payload: &[u8]) -> Result<Vec<u8>, Error> {
        if payload.len() < 106 {
            return Err(Error::Generic("Payload too short".to_string()));
        }
        if payload[0] != forge::PREFIX || payload[1] != forge::VERSION {
            return Err(Error::Generic("Invalid prefix or version".to_string()));
        }

        let encrypted_outer_seal = &payload[2..62];
        let outer_iv = &payload[62..74];
        let nonce = &payload[74..106];
        let ciphertext = &payload[106..];

        let (child_key, _) = self.derive_session_key(Some(nonce.try_into().map_err(|e| Error::AES("Trying to instantiate invalid 32 byte child key!".to_string()) )?));
        let seal_cipher = Aes256Gcm::new_from_slice(&child_key).map_err(|e| Error::AES(e.to_string()) )?;
        let inner_seal = seal_cipher
            .decrypt(Nonce::from_slice(outer_iv), encrypted_outer_seal)
            .map_err(|_| Error::Generic("Decryption failed: wrong key or corrupted data".to_string()))?;

        let msg_key = &inner_seal[0..32];
        let iv = &inner_seal[32..44];
        let msg_cipher = Aes256Gcm::new_from_slice(msg_key).map_err(|e| Error::AES(e.to_string()) )?;
        msg_cipher
            .decrypt(Nonce::from_slice(iv), ciphertext)
            .map_err(|_| Error::AES("Message decryption failed".to_string()))
    }

    /// Derive child from shard secret
    fn derive_session_key(&self, previous_nonce: Option<[u8; 32]>) -> ([u8; 32], [u8; 32]) {
        let nonce = forge::get_nonce(previous_nonce);
        let shared_secret = self.user_key.secret.diffie_hellman(&self.public_key);
        let mut key = [0u8; 32];
        Hkdf::<Sha256>::new(None, &shared_secret.to_bytes())
            .expand(&nonce, &mut key)
            .expect("HKDF expansion failed");
        (key, nonce)
    }

}


