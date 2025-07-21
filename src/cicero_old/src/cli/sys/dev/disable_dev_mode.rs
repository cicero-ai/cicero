

use std::collections::HashMap;
use falcon_cli::help::CliHelpScreen;
use falcon_cli::*;
use crate::utils::process_manager::ProcessManager;
use crate::server::config::CiceroServerConfig;
use std::fs;
use log::info;
use crate::server::CONFIG;

#[derive(Default)]
pub struct SysDisableDevMode {}

impl CliCommand for SysDisableDevMode {

    fn process(&self, args: Vec<String>, flags: Vec<String>, value_flags: HashMap<String, String>) {

        // Heading
        cli_header("Disable Developer Mode");
        if !cli_confirm("This will cause Cicero to stop dynamically loading any plugins you have under development.  Are you sure you wish to disable developer mode?") {
            return;
        }



        // Update config
        let mut config = CiceroServerConfig::new();
        config.general.plugin_dev_dir = String::new();
        config.save();

        // Restart apoolo
        let mut mgr = ProcessManager::new();
        mgr.stop("apollo");
        mgr.start("apollo", CONFIG.daemons.apollo.1 as i16, 1);

        // Success
        cli_send!("Successfully disabled developer mode.  Cicero will no longer scan for and load plugins currently under development on this machine.\n\n");
    }

    fn help(&self) -> CliHelpScreen {

        let mut help = CliHelpScreen::new(
            "Disable Developer Mode",
            "cicero disable-dev-mode",
            "Disables developer mode and Cicero will begin dynamically loading any plugins under development."
        );

        help.add_example("cicero disable-dev-mode");
        help
    }

}


