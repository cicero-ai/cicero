#![allow(warnings)]//#![allow(warnings)]
use std::collections::HashMap;
use std::time::Instant;
use lazy_static::lazy_static;
use std::env;
use falcon_cli::*;
use crate::apollo::{CiceroServerConfig, ServerMode};
use log::{info, LevelFilter};
use cicero::logger;
//use crate::cli::profile::ProfileCreate;
pub use cicero::Error;

lazy_static! {
    pub static ref CONFIG: CiceroServerConfig = CiceroServerConfig::new();
}

pub mod apollo;
mod cfx;
mod cli;
mod llm;
mod user;
mod test;

fn main() {

    // Initialize logger
    logger::init(LevelFilter::Info);

    // Check for setup
    if CONFIG.api_keys.is_empty() { }

    // Start server, if needed
    if env::args().collect::<Vec<String>>().contains(&"-d".to_string()) {
        apollo::start_server();
        return;
    }

    // Boot CLI commands, and run necessary command
    let router = cli::boot();
    cli_run(&router);
}


