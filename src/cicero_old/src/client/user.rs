
use serde_derive::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::server::security::UserKey;
use crate::server::apollo::structs::CreateUserResponse;
use x25519_dalek::PublicKey;
use crate::utils::sys;
use std::path::Path;
use std::fs;
use crate::server::security::forge;
use crate::error::Error;
use crate::utils::api_client;
use crate::llm::chat::Conversation;
use atlas_http::HttpBody;
use log::error;
use hex;

#[derive(Clone, Serialize, Deserialize)]
pub struct ClientUser {
    pub uuid: Uuid,
    pub name: String,
    pub nickname: String,
    pub email: String,
    pub language: String,
    pub auto_login: bool,
    #[serde(skip_serializing, skip_deserializing)]
    pub lock: Option<[u8; 32]>,
    pub cicero_email: String,
    pub creation_time: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub apollo_api_key: String,
    pub cicero_public_key: PublicKey,
    pub security_key: UserKey
}

impl ClientUser {

    /// Create new user
    pub fn create(name: &str, password: &str, email: &str, auto_login: &bool) -> Result<Self, Error> {

        // Generate new key
        let userkey = UserKey::generate();

        // Set params
        let mut body = HttpBody::empty();
        body.add_param("name", &name);
        body.add_param("email", &email);
        body.add_param("public_key", &hex::encode(userkey.public.to_bytes()).as_str());

        // Send RPC call
        let res = api_client::send_body::<CreateUserResponse>("v1/profile/create", "POST", &body)?;

        // Get public key
        let mut public_key_bytes: [u8; 32] = [0; 32];
        let tmp_bytes = hex::decode(res.cicero_public_key).unwrap();
        public_key_bytes.copy_from_slice(&tmp_bytes);

        // Get nickname
        let mut nickname = name.clone();
        if let Some(index) = name.find(" ") {
            nickname = &name[0..index];
        }

        // Define new user
        let mut user = ClientUser {
            uuid: res.uuid.clone(),
            name: name.to_string(),
            nickname: nickname.to_string(),
            email: email.to_string(),
            language: "en".to_string(),
            auto_login: auto_login.clone(),
            lock: Some(forge::normalize_password(&password)),
            cicero_email: String::new(),
            creation_time: Utc::now(),
            last_seen: Utc::now(),
            apollo_api_key: res.apollo_api_key.clone(),
            cicero_public_key: PublicKey::from(public_key_bytes),
            security_key: userkey
        };

        // Save user
        user.save();

        // Enable auto-login, if needed
        if *auto_login {
            let autologin_file = format!("{}/clients/.{}.login", sys::get_datadir(), user.uuid.to_string());
            fs::write(&autologin_file, hex::encode(forge::normalize_password(&password)));
        }
        Ok(user)
    }

    /// Save user
    pub fn save(&mut self) -> Result<(), Error> {

        // Get password
        let password = match &self.lock {
            Some(r) => r,
            None => {
                error!("Unable to save profile, there is no lock!");
                return Err(Error::Generic("Unable to save profile, there is no lock".to_string()));
            }
        };

        // Create parent dir
        let dbfile = format!("{}/clients/{}.dat", sys::get_datadir(), self.uuid.to_string());
        sys::prepare_parent_dir(&dbfile);

        // Serialize and encrypt
        let encoded = serde_json::to_string(&self).unwrap();
        let encrypted = forge::encrypt(&encoded.as_bytes(), &password);

        // Save file
        match fs::write(&dbfile, &encrypted) {
            Ok(_) => { },
            Err(e) => {
                error!("Unable to save profile file at {}, error: {}", dbfile, e);
                return Err(Error::Generic(format!("Unable to save profile config file, erroir: {}", e.to_string())));
            }
        };

        Ok(())
    }

    /// Load a user's profile
    pub fn load(uuid: &Uuid, password: &[u8; 32]) -> Result<Self, Error> {

        let dbfile = format!("{}/clients/{}.dat", sys::get_datadir(), uuid.to_string());
        if !Path::new(&dbfile).exists() {
            return Err(Error::Generic("No user.dat profile exists for user.".to_string()));
        }

        // Read encrypted data
        let encrypted = match fs::read(&dbfile) {
            Ok(r) => r,
            Err(e) => {
                let message = format!("Unable to read from user profile file, error: {}", e.to_string());
                return Err(Error::Generic(message.to_string()));
            }
        };

        // Decrypt
        let encoded = match forge::decrypt(&encrypted, *password) {
            Ok(r) => r,
            Err(e) => return Err(Error::InvalidPassword)
        };


        // Decode user
        let json_str = String::from_utf8(encoded).unwrap();
        let mut user: ClientUser = match serde_json::from_str(&json_str) {
            Ok(r) => r,
            Err(e) => {
                error!("User database contains corrupted data, unable to decode!  Error: {}", e);
                std::process::exit(1);
            }
        };
        user.lock = Some(*password);

        Ok(user)
    }

}

