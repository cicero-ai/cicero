
use std::path::Path;
use std::fs;
use std::collections::HashMap;
use falcon_cli::*;
use serde::{Serialize, Deserialize};
use cicero::utils::sys;
use crate::echo::setup::first_time;
use crate::Error;
use log::{info, error};

#[derive(Default, Serialize, Deserialize)]
pub struct CiceroClientConfig {
    pub general: ConfigGeneral,
    pub daemon: ConfigDaemon,
    pub api_keys: HashMap<String, String>
}

#[derive(Default, Serialize, Deserialize)]
pub struct ConfigGeneral {
    pub apollo_host: String,
    pub apollo_port: u16
}

#[derive(Default, Serialize, Deserialize)]
pub struct ConfigDaemon {
    pub host: String,
    pub port: u16
}

impl CiceroClientConfig {
    pub fn new() -> Self {

        // Setup datadir
        let config_file = format!("{}/client/config.yml", sys::get_datadir());
        if !Path::new(&config_file).exists() {
            if let Err(e) = first_time::run() {
                cli_error!("An error occured during setup, aborting: {}", e);
                std::process::exit(1);
            }
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
        info!("Successfully loaded configuration file {}", config_file);

        config
    }

    /// Save configuration
    pub fn save(&self) {

        let config_file = format!("{}/client/config.yml", sys::get_datadir());
        let yaml_str = serde_yaml::to_string(&self).unwrap();

        match fs::write(&config_file, &yaml_str) {
            Ok(_) => { },
            Err(e) => {
                error!("Unable to write to configuratoni file, {}, error: {}", config_file, e.to_string());
                std::process::exit(0);
            }
        };
    }

}



