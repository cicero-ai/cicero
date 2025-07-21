#![allow(warnings)]//#![allow(warnings)]
use lazy_static::lazy_static;
use std::env;
use falcon_cli::*;
use log::{info, LevelFilter};
use cicero::logger;

pub use crate::config::CiceroClientConfig;
pub use cicero::Error;

lazy_static! {
    pub static ref CONFIG: CiceroClientConfig = CiceroClientConfig::new();
}

mod config;
pub mod echo;
//mod cfx;
mod cli;
mod test;

fn main() {

    // Initialize logger
    logger::init(LevelFilter::Info);

    // Check for setup
    if CONFIG.general.apollo_host.is_empty() { }

    // Start server, if needed
    if env::args().collect::<Vec<String>>().contains(&"-d".to_string()) {
        //apollo::start_server();
        return;
    }

    // Boot CLI commands, and run necessary command
    let router = cli::boot();
    cli_run(&router);
}



