
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, MAIN_SEPARATOR};

pub struct Config {
    pub datadir: String,
    pub vocab_dir: String,
    pub hq_provider_key: String,
    pub hq_api_key: String,
    pub hq_api_secret: String
}

/// Load
pub fn load() -> Config {

    // Get saved config
    let saved = get_saved_config();

    // Start config
    Config {
        datadir: get_dir("datadir", "data", &saved),
        vocab_dir: get_dir("vocab_dir", "vocab", &saved),
        hq_provider_key: saved.get("hq_provider_key").unwrap_or(&"".to_string()).to_string(),
        hq_api_key: saved.get("hq_api_key").unwrap_or(&"".to_string()).to_string(),
        hq_api_secret: saved.get("hq_api_secret").unwrap_or(&"".to_string()).to_string()
    }
}

/// Get saved configuration
fn get_saved_config() -> HashMap<String, String> {

    // Get config file
    let config_file = get_config_file();
    if config_file.is_empty() {
        return HashMap::new();
    }

    // Get file contents
    let file_str = match fs::read_to_string(&config_file) {
        Ok(r) => r,
        Err(e) => {
            println!("Unable to read config file at '{}', error: {}", config_file, e);
            std::process::exit(1);
        }
    };

    // Read config file
    let mut vars: HashMap<String, String> = HashMap::new();
    for line in file_str.split("\n") {
        if line.is_empty() || !line.chars().next().unwrap().is_ascii_lowercase() {
            continue;
        }

        if let Some(index) = line.find("=") {
            let name = line[..index].trim().to_string();
            vars.insert(name, line[index+1..].trim().to_string());
        }
    }

    vars
}

/// Get configuraotin file name
fn get_config_file() -> String {
    // 1. Check command line arguments
    let args: Vec<String> = env::args().collect();
    for i in 0..args.len() - 1 {
        if args[i] == "--conf" {
            let file = &args[i + 1];
            if fs::metadata(file).is_ok() {
                return file.clone();
            }
        }
    }

    // 2. Check environment variable
    if let Ok(file) = env::var("VERAX_CONF_FILE") {
        if fs::metadata(&file).is_ok() {
            return file;
        }
    }

    // 4. Check current working directory
    let file = Path::new("verax.conf");
    if fs::metadata(file).is_ok() {
        return file.to_str().unwrap().to_string();
    }

    // 5. Return empty string if no config file found
    String::new()
}


/// Get directory
fn get_dir(name: &str, default: &str, saved: &HashMap<String, String>) -> String {

    let dirname = match saved.get(&name.to_string()) {
        Some(r) => r.to_string(),
        None => default.to_string()
    };

        if !Path::new(&dirname).is_dir() {
        println!("ERROR: The '{}' directory does not exist at: {}\n\n", name, dirname);
        std::process::exit(1);
    }

    dirname
}


