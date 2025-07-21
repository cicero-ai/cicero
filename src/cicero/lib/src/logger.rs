
use env_logger::{Builder, Target};
use log::{info, LevelFilter};
use std::fs::OpenOptions;
use std::io::Write;
use std::env;

/// Initialize standard logger, outputting only to logfile
pub fn init(mut log_level: LevelFilter) {

    if env::args().collect::<Vec<String>>().contains(&"-x".to_string()) {
        log_level = log::LevelFilter::Trace;
    } else if env::args().collect::<Vec<String>>().contains(&"-v".to_string()) {
        log_level = log::LevelFilter::Debug;
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
            //println!("{}: {} ({}:{})", record.level(), record.args(), record.file().unwrap_or("unknown"), record.line().unwrap_or(0));
            writeln!(buf, "[{}] {}: {} ({}:{})", timestamp, record.level(), record.args(), record.file().unwrap_or("unknown"), record.line().unwrap_or(0))
        })
        .filter(None, log_level)
        .target(Target::Pipe(Box::new(log_file)))
        .init();

}


// Initialize logger outputting to both, logfile and stdout
pub fn init_stdout(mut log_level: LevelFilter) {

    if env::args().collect::<Vec<String>>().contains(&"-x".to_string()) {
        log_level = log::LevelFilter::Trace;
    } else if env::args().collect::<Vec<String>>().contains(&"-v".to_string()) {
        log_level = log::LevelFilter::Debug;
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



