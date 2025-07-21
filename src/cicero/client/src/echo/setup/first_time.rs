
use local_ip_address::local_ip;
use cicero::utils::sys;
use falcon_cli::*;
use super::discover;
use crate::Error;


/// Run through first time setup
pub fn run() -> Result<(), Error> {

    let lookup: Option<(String, u16)> = match local_ip() {
        Ok(local_ip) => discover::run(local_ip.to_string()),
        Err(_) => prompt_for_apollo()
    };

    println!("lookup {:?}", lookup);
    std::process::exit(0);
    Ok(())
}

fn prompt_for_apollo() -> Option<(String, u16)> {

    None
}

