

use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::path::Path;
use std::fs;
use cicero::security::{forge, UserKey};
use cicero::utils::{random, sys};
use crate::llm::chat::Conversation;
use super::ApolloApiKey;
use crate::Error;
use log::error;

/// Server's data store and configuration.
/// Serialized via serde_json, and encrypted with double layered aes-gcm.
/// (note: would prefer to serialize via bincode, but errored out for unknown reason -- maybe the encryption?)
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Vault {
    pub hq_uuid: Uuid,
    pub creation_time: DateTime<Utc>,
    #[serde(skip_serializing)]
    lock: Option<[u8; 32]>,
    pub chat_key: UserKey,
    pub apollo_api_keys: HashMap<String, ApolloApiKey>,
    pub profiles: HashMap<Uuid, String>,
    pub helios_endpoints: HashMap<String, String>,
}

impl Vault {

    /// Create new, run during setup
    pub fn create(password: &str, uuid: &Uuid) -> Self {

        Self {
            hq_uuid: *uuid,
            creation_time: Utc::now(),
            lock: Some(forge::normalize_password(&password)), 
            chat_key: UserKey::generate(),
            ..Default::default()
        }
    }

    /// Save to disk.  Encrypted with double layer aes-gcm, serializes via bincode crate. 
    /// Saves locally to ~/.config/cicero/server/info.dat
    pub fn save(&self) -> Result<(), Error> {

        if self.lock.is_none() {
            return Err( Error::Generic("Unable to save vault, lock is missing!".to_string()) );
        }
        let password = self.lock.as_ref().unwrap();

        // Serialize and encrypt
        let encoded = serde_json::to_string(&self)
            .map_err(|e| Error::Generic(format!("Unable to serialize vault via bincode, error: {}", e)) )?;
        let encrypted = forge::encrypt(&encoded.as_bytes(), password)?;

        // Save file
        let dbfile = format!("{}/server/info.dat", sys::get_datadir());
        fs::write(&dbfile, encrypted)
            .map_err(|e| Error::Generic( format!("Unable to save vault to file {}, error: {}", dbfile, e)) )?;

        Ok(())
    }

    /// Load and decrypt the vault from local hard drive
    pub fn open() -> Result<Self, Error> {

        // Get password
        let password = Self::get_boot_password()?;

        // Get encrypted filename, ensure it exists
        let dbfile = format!("{}/server/info.dat", sys::get_datadir());
        if !Path::new(&dbfile).exists() {
            return Err( Error::Vault( format!("No vault file exists, it seems to have disappeared!  If necessary, delete configuration file at {} and restart Cicero to re-install.", sys::get_datadir()) ));
        }

        // Read the file
        let encrypted_vault = fs::read(&dbfile)?;

        // Decrypt vault
        let norm_password: [u8; 32] = forge::normalize_password(&password.as_str());
        let encoded = forge::decrypt(&encrypted_vault, norm_password)?;

        // Deserialize vault
        let mut vault: Vault = serde_json::from_slice(&encoded)
            .map_err(|e| Error::Vault( format!("Unable to deserialize vault, error {}", e)) )?;

        vault.lock = Some(norm_password);
        Ok(vault)
    }

    /// Get boot password
    pub fn get_boot_password() -> Result<String, Error> {
        let datadir = sys::get_datadir();
        let password_file = format!("{}/server/.password", datadir);
        let otp_file = format!("{}/server/.otp", datadir);

        if let Ok(pwd) = fs::read_to_string(&password_file) {
            Ok(pwd.trim().to_string())
        } else if let Ok(pwd) = fs::read_to_string(&otp_file) {
            let _ = fs::remove_file(&otp_file); // Ignore deletion failure
            Ok(pwd.trim().to_string())
        } else {
            Err( Error::Vault("No vault otp or other password file exists!".to_string()) )
        }
    }

    /// Generate Apollo API key

    pub fn generate_apollo_api_key(&mut self, uuid: Option<Uuid>) -> String {

        let api_key = random::generate_api_key(36);
        let key = ApolloApiKey {
            key: api_key.clone(),
            uuid: uuid.clone(),
            creation_time: Utc::now(),
            last_seen: Utc::now()
        };

        // Add to vault
        self.apollo_api_keys.insert(api_key.clone(), key);
        api_key
    }

}



