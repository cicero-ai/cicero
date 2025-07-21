
use std::net::TcpStream;
use std::path::Path;
use std::fs;
use falcon_cli::*;
use uuid::Uuid;
use cicero::utils::{random, sys, ProcessManager};
use crate::apollo::{Vault, CiceroServerConfig, NetworkMode};
use super::{network_mode, llm, ssl_cert};
use crate::Error;
use log::{error, info};

// Run setup, automatically called when no Apollo configuration file found on disk
pub fn run() -> Result<(), Error> {

    // Check is pollo already running
    if let Ok(_) = TcpStream::connect("127.0.0.1:7511") {
        cli_send!("There is a server running on port 7511 indicating Cicero is already running on this machine, but no server configuration file exists.  Please close the process and try again.");
        std::process::exit(1);
    }

    // Header
    cli_header("Cicero Setup");
    cli_send!("Welcome to Cicero, your new AI home assistant!  Let's quickly get you up and running.\n");
    cli_get_input("Press enter to continue, or Ctrl+C to exit.\n", "");
    println!("");

    // Create datadir
    let datadir = create_datadir();
    info!("Created directory {}", datadir);

    // Get network mode and IP address
    let (network_mode, network_ip) = network_mode::determine();

    // Get conversation llm provider, save chat router
    let uuid = Uuid::new_v4();
    llm::get_provider(&uuid);

    // Create server configuration
    let config = save_config(&datadir, network_mode, network_ip)?;

    // Create vault
    let mut vault = create_vault(&uuid);

    // Success message
    success_message(&config);

        Ok(())
}

/// Create data directory
fn create_datadir() -> String {

    // Get datadir
    let datadir = sys::get_datadir();

    // Create sub-directories
    for subdir in vec!["server", "server/ssl", "client"] {
        let dirname = format!("{}/{}", datadir, subdir);
        if Path::new(&dirname).exists() {
            continue;
        }

        // Create dir, if needed
        if let Err(e) = fs::create_dir_all(&dirname) {
            cli_error!("Unable to create local data directory, {}.  Error: {}", dirname, e);
            std::process::exit(1);
        }
    }

    datadir
}

/// Create server configuration
fn save_config(datadir: &str, network_mode: NetworkMode, ipaddr: String) -> Result<CiceroServerConfig, Error> {

    // Generate SSL certs
    ssl_cert::generate(&datadir);
    info!("Generated self signed SSL certificate");

    // Get default
    let mut config = CiceroServerConfig::default();

    // General config
    config.general.language = "en".to_string();

    // Set daemon
    config.daemon.network_mode = network_mode;
    config.daemon.host = ipaddr;
    config.daemon.https_port = 7511;
    config.daemon.ws_port = 7512;

    // Save config
    config.save();
    Ok(config)
}

fn create_vault(uuid: &Uuid) {

    // Save .password file
    let password = random::generate_password(32);
    let password_file = format!("{}/server/.password", sys::get_datadir());
    if let Err(e) = fs::write(&password_file, &password) {
        error!("Unable to write to file at {}, error: {}", password_file, e);
        std::process::exit(1);
    }

    // Create and save vault
    let mut vault = Vault::create(&password, &uuid);
    vault.save();
}

/// Finish
fn success_message(config: &CiceroServerConfig) -> Result<(), Error> {

    // Start apollo
    let mut proc_manager = ProcessManager::new()?;
    if let Err(e) = proc_manager.start("cicero-server", &["-d", "-p", "7511"]) {
        cli_send!("Failed to start Apollo server: {}\n", e);
        cli_send!("You must manually start the Apollo server by running: cicero-server -d\n");
    }

    cli_header("Cicero Setup Complete!");
    cli_send!("Great, Cicero is ready! An encryption password has been generated and saved to:\n    {}/server/.password\n", sys::get_datadir());
    cli_send!("You’re good to go as is, but if enhanced security is preferred, save this password elsewhere and delete the file - Cicero will prompt for it on boot.\n");

    cli_send!("Will this machine be a Cicero server only, or also a client?\n");

    let mut options = IndexMap::new();
    options.insert("1".to_string(), "Create a profile on this machine".to_string());
    options.insert("2".to_string(), "Server only—connect from another device".to_string());

    let option = cli_get_option("How would you like to proceed?", &options);
    if option == "2" {
        cli_send!("Nice! Connect from other devices using host {} and port 7511.\n", config.daemon.host);
    } else {
        cli_send!("All set! The Cicero client should’ve opened and started a chat. If not, in terminal run:\n    cicero\n");
        std::process::Command::new("cicero").spawn().ok(); // Launch PWA
    }

    #[cfg(target_os = "linux")]
    cli_send!("Note: For long-term use, make Cicero a service with:\n    sudo ./cicero-server sys finalize-setup\n");

    std::process::exit(0);
}


