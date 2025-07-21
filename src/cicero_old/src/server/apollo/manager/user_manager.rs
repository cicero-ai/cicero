
use atlas_http::{HttpRequest, HttpResponse, HttpBody};
use serde_derive::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use uuid::Uuid;
use x25519_dalek::PublicKey;
use crate::server::apollo::structs::CreateUserResponse;
use std::collections::hash_map::Entry;
use crate::llm::FaissIndex;
use crate::error::Error;
use crate::utils::sys;
use crate::server::apollo::user::ServerUser;
use crate::server::security::{UserKey, Postman};
use std::path::Path;
use std::fs;
use crate::server::{api, CONFIG};
use std::sync::{Arc, Mutex};
use super::Vault;
use log::error;
use hex;

pub struct UserManager { 
    users: HashMap<Uuid, Arc<Mutex<ServerUser>>>
}

impl UserManager {

    pub fn new() -> Self {
        Self { 
            users: HashMap::new()
        }
    }

    /// Create new profile from HTTP request
    pub fn create_from_http(&mut self, body: &HttpBody, vault: &mut Vault) -> HttpResponse {

        // Check required
        if let Some(key) = api::check_required(&body.params(), vec!["name","email", "public_key"]) {
            return api::response(400, format!("Missing parameter, {}", key).as_str(), String::new());
        }

        // Set variables
        let name = body.params().get("name").unwrap().clone();
        let email = body.params().get("email").unwrap().clone();

        // Get public key bytes
        let mut public_key_bytes: [u8; 32] = [0; 32];
        let tmp_bytes = hex::decode(body.params().get("public_key").unwrap()).unwrap();
        public_key_bytes.copy_from_slice(&tmp_bytes);
        let public_key = PublicKey::from(public_key_bytes);

        // Check for duplicate profile
        let existing_names: Vec<String> = vault.profiles.values().map(|n| n.to_string()).collect();
        if existing_names.contains(&name) {
            return api::response(400, "Account with that name already exists on the machine", String::new());
            }
        // Generate Apollo api key

        let uuid = Uuid::new_v4();
        let api_key = vault.generate_apollo_api_key(Some(uuid.clone()));

        // Define profile
        let user = match ServerUser::create(&uuid, &name.as_str(), &email.as_str(), &public_key, &api_key, &vault.chat_key) {
            Ok(r) => r,
            Err(e) => return api::response(400, &e.to_string().as_str(), String::new())
        };

        // Save user
        self.save(&user, &vault.chat_key);

        // Add to vault
        vault.profiles.insert(user.uuid.clone(), user.name.clone());
        vault.save();

        // Get response
        let res = CreateUserResponse {
            uuid: uuid.clone(),
            apollo_api_key: api_key.clone(),
            cicero_public_key: hex::encode(vault.chat_key.public.to_bytes())
        };

        self.users.insert(user.uuid.clone(), Arc::new(Mutex::new(user)));
        api::response(200, "", res)
    }

    /// Save user profile
    pub fn save(&mut self, user: &ServerUser, server_key: &UserKey) {

        // Initialize
        let user_file = format!("{}/profiles/{}/user.dat", sys::get_datadir(), user.uuid.to_string());
        let mut postman = server_key.to_postman(&user.public_key);

        // Encrypt user
        let encoded = serde_json::to_string(&user).unwrap();
        let enc_profile = postman.encrypt(&encoded.as_bytes());

        // Save file
        sys::prepare_parent_dir(&user_file);
        fs::write(&user_file, &enc_profile).unwrap();
        //self.users.insert(user.uuid.clone(), user);
    }

    /// Check if user already logged in
    pub fn is_logged_in(&self, uuid: &Uuid) -> bool {
        self.users.contains_key(uuid)
    }

    /// Load user profile, if not alredy logged in, decrypt profile database and login
    pub fn login(&mut self, uuid: &Uuid, public_key: &PublicKey, server_key: &UserKey) -> Result<Option<Arc<Mutex<ServerUser>>>, Error> {

        // Check if already loadded
        //if self.users.contains_key(&uuid) {
            //return Ok(self.users.get(&uuid).unwrap().clone());
        //}

        // Check if exists
        let user_file = format!("{}/profiles/{}/user.dat", sys::get_datadir(), uuid.to_string());
        if !Path::new(&user_file).exists() {
            return Err(Error::Generic("No user exists with that uuid".to_string()));
        }

        // Load and decrypt
        let enc_profile = fs::read(&user_file).unwrap();
        let mut postman = server_key.to_postman(&public_key);
        let encoded = postman.decrypt(&enc_profile)?;

        // Deserialize
        let json_str = String::from_utf8(encoded).unwrap();
        let mut user: ServerUser = match serde_json::from_str(&json_str) {
            Ok(r) => r,
            Err(e) => {
                error!("User profile database file contains corrupted data, unable to decode!  Error: {}", e);
                return Err(Error::Generic(format!("User profile database file contains corrupted data, unable to decode!  Error: {}", e)));
            }
        };

        // Load faiss index
        let faiss_datadir = format!("{}/profiles/{}/faiss/profile", sys::get_datadir(), uuid.to_string()); 
        user.faiss = FaissIndex::new(&faiss_datadir.as_str(), &server_key, &user.public_key);

        // Insert into users
        self.users.insert(uuid.clone(), Arc::new(Mutex::new(user)));
        Ok(Some(self.users.get(uuid).unwrap().clone()))
    }

    /// Get an already authenticated user
    pub fn get(&self, uuid: &Uuid) -> Arc<Mutex<ServerUser>> {
        self.users.get(uuid).unwrap().clone()
    }


}


