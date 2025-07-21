
use dirs;
use std::{env, fs};
use std::path::{Path, MAIN_SEPARATOR};
use std::net::UdpSocket;
use tch::Device;
use log::error;

use crate::CLIENT_CONFIG;

/// Get data directory
pub fn get_datadir() -> String {

    // Check if env variable set
    let datadir = match env::var("CICERO_DATADIR") {
        Ok(r) => r.to_string(),
        Err(_) => {
            let datadir = dirs::data_dir().expect("Unable to determine user's data directory on this machine.");
            format!("{}/cicero", datadir.to_str().unwrap().trim_end_matches("/"))
        }
    };

    datadir.trim_end_matches(MAIN_SEPARATOR).to_string()
}


/// Get lib directory
pub fn get_libdir() -> Option<String> {

    // Check if env variable set
    if let Ok(dirname) = env::var("CICERO_LIBDIR") {
        let filename = format!("{}/nlu/en/nlu.dat", dirname.trim_end_matches(MAIN_SEPARATOR).to_string());
        if Path::new(&filename).exists() {
            return Some(dirname.trim_end_matches(MAIN_SEPARATOR).to_string());
        }
        error!("Invalid lib directory, does not contain a ~/nlu/en.dat file: {}", dirname);
    }

    // Check cwd
    if let Ok(cwd) = env::current_dir() {
        let check_dirname: String = cwd.to_string_lossy().into_owned().trim_end_matches(MAIN_SEPARATOR).to_string();
        let check_dirs = vec![
            check_dirname.clone(),
            format!("{}/lib", check_dirname)
        ];

        for dirname in check_dirs {
            let filename = format!("{}/nlu/en.dat", dirname.trim_end_matches(MAIN_SEPARATOR).to_string());
            if Path::new(&filename).exists() {
                return Some(dirname.trim_end_matches(MAIN_SEPARATOR).to_string());
            }
        }
    }

    // Set dirs to check
    let check_dirs = vec![
        "/usr/lib/cicero",
        "/usr/local/lib/cicero",
        "/var/lib/cicero",
        "/usr/share/include/cicero"
    ];

    // Check dirs
    for dirname in check_dirs {
        let filename = format!("{}/nlu/en.dat", dirname.trim_end_matches(MAIN_SEPARATOR).to_string());
        if Path::new(&filename).exists() {
            return Some(dirname.trim_end_matches(MAIN_SEPARATOR).to_string());
        }
    }

    None
}

/// Get inference device
pub fn get_inference_device() -> Device {
    Device::Cpu
}

/// Get IP address
pub fn determine_ip_address(remote_ip: &str) -> Option<String> {

    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(r) => r,
        Err(_) => return None
    };

    match socket.connect(&remote_ip) {
        Ok(_) => { },
        Err(_) => return None
    };

    let public_ip = socket.local_addr().unwrap().ip().to_string();
    Some(public_ip)
}


/// Get apollo api key
pub fn get_apollo_api_key() -> String {

    if let Some(user) = &CLIENT_CONFIG.current_user {
        return user.apollo_api_key.clone();
    }
    CLIENT_CONFIG.apollo_api_key.clone()
}

/// Ensure parent directory is created
pub fn prepare_parent_dir(filename: &str) {
let parent_dir = Path::new(&filename).parent().unwrap();
    if Path::new(&parent_dir).exists() {
        return;
    }

    match fs::create_dir_all(&parent_dir) {
        Ok(_) => { },
        Err(e) => {
            error!("Unable to create parent directory for {}, error: {}", filename, e.to_string());
            std::process::exit(1);
        }
    };
}


/// Get verswion
pub fn get_version() {
    let major = env::var("CARGO_PKG_VERSION_MAJOR").unwrap();
    let minor = env::var("CARGO_PKG_VERSION_MINOR").unwrap();
    let patch = env::var("CARGO_PKG_VERSION_PATCH").unwrap();

    println!("Major: {}", major);
    println!("Minor: {}", minor);
    println!("Patch: {}", patch);


}


