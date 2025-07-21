
use serde_derive::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::path::Path;
use std::fs;
use crate::server::security::{forge, UserKey, ApolloApiKey};
use crate::llm::chat::Conversation;
use super::UserManager;
use crate::server::apollo::user::ServerUser;
use crate::error::Error;
use crate::utils::{random, sys};
use log::error;

#[derive(Serialize, Deserialize)]
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

    /// Create new, during setup
    pub fn create(password: &str, uuid: &Uuid, gpu_device_id: Option<usize>) -> Self {

        Self {
            hq_uuid: *uuid,
            creation_time: Utc::now(),
            lock: Some(forge::normalize_password(&password)), 
            chat_key: UserKey::generate(),
            apollo_api_keys: HashMap::new(),
            profiles: HashMap::new(),
            helios_endpoints: HashMap::new()
        }
    }

    /// Save 
    pub fn save(&mut self) {

        // Get password
        let password = match &self.lock {
            Some(r) => r,
            None => {
                error!("Unable to save vault, there is no lock!");
                std::process::exit(1);
            }
        };

        // Serialize and encrypt
        // Encode it
        let encoded = serde_json::to_string(&self).unwrap();
        let encrypted = forge::encrypt(&encoded.as_bytes(), password);

        // Save file
        let dbfile = format!("{}/manager/info.dat", sys::get_datadir());
        match fs::write(&dbfile, &encrypted) {
            Ok(_) => { },
            Err(e) => {
                error!("Unable to save main config file at {}, error: {}", dbfile, e);
                std::process::exit(1);
            }
        };
    }

    /// Load vault
    pub fn open() -> Option<Self> {

        // Get password
        let password = match Self::get_boot_password() {
            Some(r) => r,
            None => {
                error!("No encryption password provided to daemon, exiting.");
                std::process::exit(1);
            }
        };

        // Get encrypted file
        let dbfile = format!("{}/manager/info.dat", sys::get_datadir());
        if !Path::new(&dbfile).exists() {
            error!("No vault file exists!  It seems to have disappeared.");
            std::process::exit(1);
        }
        let encrypted_vault = fs::read(&dbfile).expect("Unable to read vault file");
        let norm_password: [u8; 32] = forge::normalize_password(&password.as_str());

        // Decrypt 
        let encoded = match forge::decrypt(&encrypted_vault, norm_password) {
            Ok(r) => r,
            Err(e) => {
                error!("Invalid encryption password, aborting.");
                std::process::exit(0);
            }
        };

        // Decode vault
        let json_str = String::from_utf8(encoded).unwrap();
        let mut vault: Vault = match serde_json::from_str(&json_str) {
            Ok(r) => r,
            Err(e) => {
                error!("Vault contains corrupted data, unable to decode!  Error: {}", e);
                std::process::exit(1);
            }
        };
        vault.lock = Some(norm_password.clone());

        Some(vault)
    }

    /// Get boot password
    pub fn get_boot_password() -> Option<String> {

        // Set password files
        let password_file = format!("{}/manager/.password", sys::get_datadir());
        let otp_file = format!("{}/manager/.otp", sys::get_datadir());

        // Look for password
        let mut password: Option<String> = None;
        if Path::new(&password_file).exists() {
            password = Some(fs::read_to_string(&password_file).expect("Unable to read .password file").trim().to_string());
        } else if Path::new(&otp_file).exists() {
            password = Some(fs::read_to_string(&otp_file).expect("Unable to read .otp file").trim().to_string());
            fs::remove_file(&otp_file).expect("Unable to delete .otp file");
        }

        password
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


