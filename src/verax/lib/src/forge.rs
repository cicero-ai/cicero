
use aes_gcm::aead::{Aead, AeadCore, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Nonce, Key};
use rand::Rng;
use rand::RngCore;
use argon2::Argon2;
use sha2::{Sha256, Digest};
use hkdf::Hkdf;
use crate::error::Error;
use std::convert::TryInto;

pub static PREFIX: u8 = 0x43;
pub static VERSION: u8 = 0x01;

/// Encrypt
pub fn encrypt(message: &[u8], password: &[u8; 32]) -> Vec<u8> {

    // Generate encrypt key and iv
    let mut rng = rand::thread_rng();
    let encryption_key: [u8; 32] = rng.gen();
    let iv: [u8; 12] = rng.gen();

    // Encrypt the message
    let key: &Key<Aes256Gcm> = &encryption_key.into();
    let cipher = Aes256Gcm::new(&key);
    let ciphertext = cipher.encrypt(&iv.into(), message.as_ref()).unwrap();

    // Concatenate the encryption key and IV
    let full_key = [&encryption_key[..], &iv[..]].concat();
    let password_iv: [u8; 12] = rng.gen();

    // Argon2 hash, and derive child key
    let (argon_hash, salt) = argon2_hash(&password, None);
    let (child_key, nonce) = derive_child(&argon_hash, None);

    // Encrypt full key with supplied password
    let outer_key = Key::<Aes256Gcm>::from(child_key.clone());
    let outer_cipher = Aes256Gcm::new(&outer_key);
    let encrypted_full_key = outer_cipher.encrypt(&password_iv.into(), full_key.as_ref()).unwrap();

    // Generate the concatenated header
    let mut header: Vec<u8> = Vec::new();
    header.push(PREFIX);
    header.push(VERSION);
    header.extend_from_slice(&encrypted_full_key);
    header.extend_from_slice(&password_iv);
    header.extend_from_slice(&nonce);
    header.extend_from_slice(&salt);

    // Prepend the header to the encrypted message
    [header, ciphertext].concat()
}

/// Encrypt with str password
pub fn encrypt_with_str(message: &[u8], password: &str) -> Vec<u8> {
    let norm_password = normalize_password(&password);
    encrypt(&message, &norm_password)
}
/// Decrypt
pub fn decrypt(payload: &[u8], password: [u8; 32]) -> Result<Vec<u8>, Error> {

    // Check header
    if payload[0] != PREFIX {
        return Err(Error::Generic("Invalid message prefix.".to_string()));
    } else if payload[1] != VERSION {
        return Err(Error::Generic("Invalid message version.".to_string()));
    }

    // Define empty arrays
    let mut password_iv: [u8; 12] = [0; 12];
    let mut nonce: [u8; 32] = [0; 32];
    let mut salt: [u8; 16] = [0; 16];

    // Get password iv, nonce, and salt
    password_iv.copy_from_slice(&payload[62..74]);
    nonce.copy_from_slice(&payload[74..106]);
    salt.copy_from_slice(&payload[106..122]);

    // Argon2 hash and derive child
    let (argon_hash, _) = argon2_hash(&password, Some(salt));
    let (child_key, _) = derive_child(&argon_hash, Some(nonce));

    // Get encryption key
    let key = Key::<Aes256Gcm>::from_slice(&child_key);

    // Decrypd seal
    let cipher = Aes256Gcm::new(&key);
    let inner_seal = match cipher.decrypt(&password_iv.into(), payload[2..62].as_ref()) {
        Ok(r) => r,
        Err(e) => return Err( Error::Generic("Invalid encryption password.".to_string())) 
    };

    // Get iv and encryption key
    let mut iv: [u8; 12] = [0; 12];
    iv.copy_from_slice(&inner_seal[32..44]);
    let msg_key = Key::<Aes256Gcm>::from_slice(&inner_seal[0..32]);

    // Decrypt message
    let msg_cipher = Aes256Gcm::new(&msg_key);
    let message = match msg_cipher.decrypt(&iv.into(), payload[122..].as_ref()) {
        Ok(r) => r,
        Err(e) => return Err( Error::Generic("Invalid encryption password.".to_string())) 
    };

    Ok(message)
}

/// Decrypt with password &str
pub fn decrypt_with_str(payload: &[u8], password: &str) -> Result<Vec<u8>, Error> {
    let norm_password = normalize_password(&password);
    decrypt(&payload, norm_password)
}

/// Ensure password is  32 byte array
pub fn normalize_password(password: &str) -> [u8; 32] {

    // SHA256 password
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let hash = hasher.finalize();

    let mut output: [u8; 32] = [0; 32];
    output.copy_from_slice(&hash);

    output
}

/// Hash via Argon2
fn argon2_hash(password: &[u8; 32], previous_salt: Option<[u8; 16]>) -> ([u8; 32], [u8; 16]) {

    // Check if we have salt
    let mut salt: [u8; 16] = [0; 16];
    if let Some(prev_salt) = previous_salt {
        salt = prev_salt;
    } else {
        let mut rng = OsRng;
        rng.fill_bytes(&mut salt);
    }

    // Hash the password
    let mut res_hash: [u8; 32] = [0; 32];
    let argon2 = Argon2::default();
    argon2.hash_password_into(&password.to_vec(), &salt, &mut res_hash);

    (res_hash, salt)
}

// Derive child
fn derive_child(password: &[u8; 32], previous_nonce: Option<[u8; 32]>) -> ([u8; 32], [u8; 32]) {

    // Generate nonce
        let nonce: [u8; 32] = get_nonce(previous_nonce);

    // Derive child key from nonce
    let mut child_bytes = [0u8; 32];
    Hkdf::<Sha256>::from_prk(&password.to_vec())
        .expect("Unable to derive child key")
        .expand(&nonce, &mut child_bytes)
        .unwrap();

    (child_bytes, nonce)
}

pub fn get_nonce(previous_nonce: Option<[u8; 32]>) -> [u8; 32] {

    // Generate nonce
    let mut nonce: [u8; 32] = [0; 32];
    if let Some(prev_nonce) = previous_nonce {
        nonce = prev_nonce;
    } else {
        let mut rng = OsRng;
        rng.fill_bytes(&mut nonce);
    }

    nonce
}


