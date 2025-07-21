
use crate::server::apollo::manager::setup;
use crate::server::config::ConfigDaemons;
use std::path::Path;
use std::collections::HashMap;
use std::fs;
use serde_derive::{Serialize, Deserialize};
use log::error;
use uuid::Uuid;
use crate::utils::{sys, ProcessManager};
use crate::CLIENT_CONFIG;
use hex;
use super::ClientUser;

#[derive(Serialize, Deserialize)]
pub struct CiceroClientConfig {
    #[serde(skip_serializing, skip_deserializing)]
    pub current_user: Option<ClientUser>,
    #[serde(skip_serializing, skip_deserializing)]
    pub is_first_time: bool,
    pub current_uuid: Option<Uuid>,
    pub cfs_dir: String,
    pub apollo_api_key: String,
    pub daemons: ConfigDaemons,
    pub local_users: HashMap<Uuid, String>
}

impl CiceroClientConfig {

    pub fn new() -> Self {

        // Check for client.yml config file
        let mut is_first_time = false;
        let config_file = format!("{}/config/client.yml", sys::get_datadir());
        if !Path::new(&config_file).exists() {
            setup::run();
            is_first_time = true
        }

        // Load yaml file
        let yaml_code = fs::read_to_string(&config_file).unwrap();
        let mut config: CiceroClientConfig = match serde_yaml::from_str(&yaml_code) {
            Ok(r) => r,
            Err(e) => {
                error!("Unable to properly deserialize the configuration file at {}, error: {}", config_file, e.to_string());
                std::process::exit(1);
            }
        };
        config.is_first_time = is_first_time;

    config.check_login();
        config
    }

    /// Set current user
    pub fn set_current_user(user: &ClientUser) {
        let mut config = Self::new();
        config.current_user = Some(user.clone());
        config.save();
    }

    /// Add local user
    pub fn add_local_user(user: &ClientUser, password: Option<[u8; 32]>) {

        // Add local user to config
        let mut config = Self::new();
        config.local_users.insert(user.uuid.clone(), user.name.clone());
        config.save();

        // Return if not making new user current logged in user
        if password.is_none() {
            return;
        }

        // Save .login file
        let filename = format!("{}/tmp/.login", sys::get_datadir());
        sys::prepare_parent_dir(&filename);
        let hex_password = hex::encode(&password.unwrap());
        fs::write(&filename, format!("{}\n{}", user.uuid.to_string(), hex_password)).expect("Unable to write to temporary ~/tmp/.login file");

        // Start echo server
        let mut manager = ProcessManager::new();
        manager.start("echo", CLIENT_CONFIG.daemons.echo.1 as i16, 1);
    }

    /// Check login
    pub fn check_login(&mut self) {

        // Check for .login file
        let filename = format!("{}/tmp/.login", sys::get_datadir());
        if !Path::new(&filename).exists() {
            return;
        }

        // Read file
        let contents = fs::read_to_string(&filename).expect("Unable to read from temporary file at ~/tmp/.login");
        let lines: Vec<String> = contents.split("\n").map(|line| line.to_string()).collect::<Vec<String>>();
        //fs::remove_file(&filename).expect("Unable to remove templorary ~/tmp/.login file");

        // Parse file contents
        let uuid = match Uuid::parse_str(&lines[0]) {
            Ok(r) => r,
            Err(_) => return
        };

        let mut password: [u8; 32] = [0; 32];
        password.copy_from_slice(&hex::decode(&lines[1]).unwrap()[0..32]);

        // Login
        let user = match ClientUser::load(&uuid, &password) {
            Ok(r) => r,
            Err(_) => return
        };

        // Update config
        self.current_uuid = Some(user.uuid.clone());
        self.current_user = Some(user);
        self.save();
    }

    /// Save config
    pub fn save(&self) {
        let config_file = format!("{}/config/client.yml", sys::get_datadir());
        sys::prepare_parent_dir(&config_file);

        let yaml_str = serde_yaml::to_string(&self).unwrap();
        fs::write(&config_file, &yaml_str).expect("Unable to write to /config/client.yml file");
    }

}

impl Default for CiceroClientConfig {
    fn default() -> CiceroClientConfig {
        CiceroClientConfig {
            current_user: None,
            is_first_time: false,
            current_uuid: None,
            cfs_dir: String::new(),
            apollo_api_key: String::new(),
            daemons: ConfigDaemons::default(),
            local_users: HashMap::new()
        }
    }
}


