

use x25519_dalek::{StaticSecret, PublicKey};
use aes_gcm::aead::{Aead, AeadCore, KeyInit, OsRng};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use crate::error::Error;
use uuid::Uuid;
use super::{forge, Postman};

#[derive(Clone, Serialize, Deserialize)]
pub struct UserKey {
    pub uuid: Uuid,
    pub is_active: bool,
    pub creation_time: DateTime<Utc>,
    pub public: PublicKey,
    pub secret: StaticSecret
}

impl UserKey {

    /// Generate a new key pair
    pub fn generate() -> UserKey {

        // Generate
        let secret = StaticSecret::new(OsRng);


        UserKey {
            uuid: Uuid::new_v4(),
            is_active: true,
            creation_time: Utc::now(),
            public: PublicKey::from(&secret),
            secret
        }
    }

    /// Get postman
    pub fn to_postman(&self, public_key: &PublicKey) -> Postman {
        Postman::new(&self, &public_key)
    }

}

impl Default for UserKey {
    fn default() -> Self {
        Self::generate()
    }
}




