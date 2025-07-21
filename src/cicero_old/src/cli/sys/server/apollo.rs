
use std::collections::HashMap;
use falcon_cli::help::CliHelpScreen;
use falcon_cli::*;
use crate::utils::process_manager::ProcessManager;
use crate::utils::sys;
use std::path::Path;
use std::fs;
use log::info;
use crate::server::CONFIG;

#[derive(Default)]
pub struct SysServerApollo {}

impl CliCommand for SysServerApollo {

    fn process(&self, args: Vec<String>, flags: Vec<String>, value_flags: HashMap<String, String>) {

        // Get action
        if args.len() == 0 {
            cli_error!("You did not specify an action to take.  Must be 'start', 'stop', 'restart'.");
            return;
        }

        // Validate action
        let action = args[0].to_lowercase().clone();
        if !vec!["start", "stop", "restart"].contains(&action.as_str()) {
            cli_error!("Invalid action specified '{}'.  Supported actions are 'start', 'stop', 'restart'", action);
            return;
        }
        let mut manager = ProcessManager::new();

        // Stop, if needed
        if action == "stop" || action == "restart" {
            manager.stop(&"apollo");
            if action == "stop" {
                return;
            }
        }

        // Get config variables
        let num_threads: i8 = value_flags.get(&"num-threads".to_string()).unwrap_or(&"1".to_string()).clone().parse::<i8>().unwrap();
        let port: i16 = value_flags.get(&"port".to_string()).unwrap_or(&CONFIG.daemons.apollo.1.to_string()).clone().parse::<i16>().unwrap();

        // Get encryption password, if needed
        let password_file = format!("{}/manager/.password", sys::get_datadir());
        if !Path::new(&password_file).exists() {
            let encrypt_password = cli_get_password("Encryption Password: ");
            let otp_file = format!("{}/manager/.otp", sys::get_datadir());
            fs::write(&otp_file, &encrypt_password).expect("Unable to write to ~/manager/.otp file");
        }

        // Start as needed
        manager.start("apollo", port, num_threads);
    }

    fn help(&self) -> CliHelpScreen {

        let mut help = CliHelpScreen::new(
            "Start / stop Apollo server", 
            "cicero apollod (start|stop|restart)",
            "Manage the Apollo server daemon process which hands the internal back-end server of Cicero."
        );

        help.add_example("cicero apollod start");
        help
    }

}


