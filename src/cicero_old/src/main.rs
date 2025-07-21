#![allow(warnings)]//#![allow(warnings)]
use falcon_cli::*;
use lazy_static::lazy_static;
use std::collections::HashMap;
use crate::cli::profile::ProfileCreate;
use crate::client::CiceroClientConfig;
use env_logger::{Builder, Target};
use log::{info, LevelFilter};
use std::io::Write;
use std::time::Instant;
use std::fs::OpenOptions;
use std::env;

lazy_static! {
    pub static ref CLIENT_CONFIG: CiceroClientConfig = CiceroClientConfig::new();
}



#[cfg(feature = "apollo")]
pub mod api;
pub mod cli;
#[cfg(feature="echo")]
pub mod client;
pub mod error;
pub mod llm;
pub mod server;
pub mod utils;
pub mod test;

fn main() {

    // Initialize logger
    init_logger();

    // Start server, if needed
    if env::args().collect::<Vec<String>>().contains(&"-d".to_string()) {
        server::start();
        return;
    }

    // Check for first user
    if CLIENT_CONFIG.is_first_time {
        create_first_user();
    }

    // Boot CLI commands
    let router = cli::boot();

    // Check for no command
    if env::args().len() == 1 {
        cli_send!("ERROR: You did not specify a command to run.  Please specify a command or use 'help' or '-h' to view a list of all available commands.\n\n");
        cli_send!("If you wish to login, run the command: cicero login\n\n");
        std::process::exit(0);
    }

    // Run CLi command
    cli_run(&router);
}

// Init logger
fn init_logger() {

    let mut log_level = log::LevelFilter::Info;
    if env::args().collect::<Vec<String>>().contains(&"-x".to_string()) {
        log_level = log::LevelFilter::Trace;
    } else if env::args().collect::<Vec<String>>().contains(&"-v".to_string()) {
        
    }

    let log_file = OpenOptions::new()
        .create(true)
        .write(true)
        //.truncate(true)
        .open("cicero.log")
        .unwrap();

    // Init logger
    Builder::new()
        .format(|buf, record| {
            let timestamp = chrono::Local::now().format("%H:%M:%S").to_string();
            println!("{}: {} ({}:{})", record.level(), record.args(), record.file().unwrap_or("unknown"), record.line().unwrap_or(0));
            writeln!(buf, "[{}] {}: {} ({}:{})", timestamp, record.level(), record.args(), record.file().unwrap_or("unknown"), record.line().unwrap_or(0))
        })
        //.filter_module("cicero", log_level)
        //.filter(None, LevelFilter::Off)
        .filter(None, log_level)
        .target(Target::Pipe(Box::new(log_file)))
        .init();

}

/// Create first user
fn create_first_user() {

    // Create profile
    let cmd = ProfileCreate { };
    cmd.process(vec![], vec!["q".to_string()], HashMap::new());

    cli_send!("Congratulations, your new profile has been created and is ready!  A secure, encrypted file vault has been created for you and Cicero to share files with each other at:\n\n");
    cli_send!("    {}\n\n", CLIENT_CONFIG.cfs_dir);
    cli_send!("This is the only directory Cicero has access to.  Anytime you need to send Cicero files to Cicero place them in this directory, and Cicero will do the same to send you completed work.\n\n");
    cli_send!("You're all set, and can begin using Cicero!  Start a chat with Cicero with the command:\n\n");
    cli_send!("    ./cicero chat\n\n");

    #[cfg(target_os = "linux")] {
        cli_send!("NOTE: For installations intended to be long-term, it's recommended you finalize setup by turning Cicero into its own Linux service under it's own user/group.  You may do so automatically by running the command:");
        cli_send!("    sudo ./cicero sys finalize-setup\n\n");
    }

    std::process::exit(0);

}


