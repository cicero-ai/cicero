#![allow(warnings)]

use lazy_static::lazy_static;
use falcon_cli::*;
use crate::config::Config;
use verax::License;

mod cli;
mod config;
mod hq;
mod packager;
mod tools;

lazy_static! {
    pub static ref CONFIG: Config = config::load();
}


fn main() {

    let mut lic = License::load();
    //lic.validate();
    println!("Done"); std::process::exit(0);
    // Load cli
    let router = cli::boot();
    cli_run(&router);
}


