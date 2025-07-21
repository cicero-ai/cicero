

use std::path::Path;
use std::fs;
use std::collections::HashMap;
use falcon_cli::*;
use serde::{Serialize, Deserialize};
use cicero::utils::sys;
use crate::apollo::setup::first_time;
use crate::Error;
use log::{info, error};

#[derive(Default, Serialize, Deserialize)]
pub struct CiceroServerConfig {
    pub general: ConfigGeneral,
    pub daemon: ConfigDaemon,
    pub api_keys: HashMap<String, String>
}

#[derive(Default, Serialize, Deserialize)]
pub struct ConfigGeneral {
    pub mode: ServerMode,
    pub plugin_dev_dir: String,
    pub language: String
}

#[derive(Default, Serialize, Deserialize)]
pub struct ConfigDaemon {
    pub network_mode: NetworkMode,
    pub host: String,
    pub https_port: u16,
    pub ws_port: u16
}

#[derive(Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum ServerMode {
    devel,
    #[default]
    prod
}

#[derive(Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum NetworkMode {
    local,
    #[default]
    lan,
    internet
}

impl CiceroServerConfig {
    pub fn new() -> Self {

        // Setup datadir
        let config_file = format!("{}/server/config.json", sys::get_datadir());
        if !Path::new(&config_file).exists() {
            if let Err(e) = first_time::run() {
                cli_error!("An error occured during setup, aborting: {}", e);
                std::process::exit(1);
            }
        }

        // Load json file
        let json_str = fs::read_to_string(&config_file).unwrap();
        let mut config: CiceroServerConfig = match serde_json::from_str(&json_str) {
            Ok(r) => r,
            Err(e) => {
                error!("Unable to properly deserialize the configuration file at {}, error: {}", config_file, e.to_string());
                std::process::exit(1);
            }
        };
        info!("Successfully loaded configuration file {}", config_file);

        config
    }

    /// Save configuration
    pub fn save(&self) {

        let config_file = format!("{}/server/config.json", sys::get_datadir());
        let json_str = serde_json::to_string(&self).expect("Unable to encode config file into JSON");

        match fs::write(&config_file, &json_str) {
            Ok(_) => { },
            Err(e) => {
                cli_error!("Unable to write to configuration file, {}, error: {}", config_file, e.to_string());
                std::process::exit(0);
            }
        };
    }

}


