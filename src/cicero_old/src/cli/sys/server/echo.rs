
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
pub struct SysServerEcho {}

impl CliCommand for SysServerEcho {

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
            manager.stop(&"echo");
            if action == "stop" {
                return;
            }
        }

        // Get config variables
        let port: i16 = value_flags.get(&"port".to_string()).unwrap_or(&CONFIG.daemons.echo.1.to_string()).clone().parse::<i16>().unwrap();

        // Start as needed
        manager.start("echo", port, 1);
    }

    fn help(&self) -> CliHelpScreen {

        let mut help = CliHelpScreen::new(
            "Start / stop Echo server", 
            "cicero echod (start|stop|restart)",
            "Manage the Echo daemon server process which is the front-end client on this machine."
        );

        help.add_example("cicero echod start");
        help
    }

}


