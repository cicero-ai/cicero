
use atlas_http::{HttpRequest, HttpResponse};
use x25519_dalek::PublicKey;
use serde_derive::{Serialize, Deserialize};
use std::path::Path;
use std::fs;
use std::sync::{Arc, Mutex};
use std::net::TcpStream;
use uuid::Uuid;
use crate::server::api;
use crate::server::apollo::nlp;
use crate::utils::sys;
use crate::llm::models::ModelLibrary;
use crate::server::apollo::user::ServerUser;
use super::{Vault, UserManager, EchoAssistant, PluginManager, ChatManager};
use log::{debug, error};

pub struct CiceroManager {
    pub vault: Vault,
    user_mgr: UserManager,
    chat_mgr: ChatManager,
    plugin_mgr: PluginManager,
    echo_assistant: EchoAssistant
}

impl CiceroManager {

    pub fn new() -> Self {

        // Open vault
        let mut vault = match Vault::open() {
            Some(r) => r,
            None => {
                error!("Unable to open vault, exiting.");
                std::process::exit(1);
            }
        };

        Self {
            vault,
            user_mgr: UserManager::new(),
            chat_mgr: ChatManager::new(),
            plugin_mgr: PluginManager:: new(),
            echo_assistant: EchoAssistant::new()
        }
    }

    /// Authenticate API request
    pub fn authenticate_api_request(&mut self, req: &HttpRequest) -> Result<Option<Arc<Mutex<ServerUser>>>, HttpResponse> {

        // Ensure header exists
        if !req.headers.has_lower("Cicero-api-key") {
            debug!("Auth:  No API key header, rejecting.");
            return Err(api::response(401, "Unauthorized.  No API key.", String::new()));
        }

        // Check API key
        let api_key = req.headers.get_lower("cicero-api-key").unwrap();
        if !self.vault.apollo_api_keys.contains_key(&api_key) {
            debug!("Auth: Provided API key does not exist in vault, rejecting.");
            return Err(api::response(401, "Unauthorized.  Invalid API key.", String::new()));
        }
        let key = self.vault.apollo_api_keys.get(&api_key).unwrap();

        // Check key assigned to uuid
        if key.uuid.is_none() {
            debug!("Auth: API key found, is not assigned to a uuid, returning as guest.");
            return Ok(None);
        } 

        // Check if we have profile
        if !self.user_mgr.is_logged_in(&key.uuid.unwrap()) {
            debug!("Auth: API key found and valid, but not currently logged in, returning as guest.");
            return Ok(None);
        }

        debug!("Auth: Successfully authenticated");
        Ok(Some(self.user_mgr.get(&key.uuid.unwrap())))
    }

    /// Login user
    pub fn login(&mut self, uuid: &Uuid, public_key: &PublicKey) -> bool {

        let user = match self.user_mgr.login(&uuid, &public_key, &self.vault.chat_key) {
            Ok(r) => r,
            Err(e) => return false
        };

        if user.unwrap().lock().unwrap().public_key != *public_key {
            return false;
        }
        true
    }

    /// Handle v1 API request
    pub fn handle_v1_api_request(&mut self, parts: &Vec<String>, req: &HttpRequest, stream: &mut TcpStream) -> HttpResponse {

        // Authenticate
        let user = match self.authenticate_api_request(&req) {
            Ok(r) => r,
            Err(res) => return res
        };

        let path = parts[1..].join("/").to_string();
        let search = (path.as_str(), req.method.as_str());
        let res: HttpResponse = match search {
            ("profile/create", "POST") => self.user_mgr.create_from_http(&req.body, &mut self.vault),
            ("echo/get_menus", "GET") => self.echo_assistant.get_menus(&mut self.plugin_mgr, user.clone()),
            ("chat/checkin", "GET") => self.chat_mgr.checkin(user.clone()),
            ("chat/reply", "POST") => self.chat_mgr.user_reply(user.clone(), &req, stream),
            _ => api::response(404, "No endpoint at this location", String::new())
        };

        // Save user, if needed
        if search == ("chat/reply", "POST") {
            let save_user = user.as_ref().unwrap().lock().unwrap();
            self.user_mgr.save(&save_user, &self.vault.chat_key);
        }

        res
    }

}



