
use std::collections::HashMap;
use falcon_cli::help::CliHelpScreen;
use falcon_cli::*;
use crate::utils::process_manager::ProcessManager;
use crate::server::config::CiceroServerConfig;
use std::path::Path;
use std::fs;
use std::path::MAIN_SEPARATOR;
use log::{info, error};
use crate::server::CONFIG;

#[derive(Default)]
pub struct SysEnableDevMode {}

impl CliCommand for SysEnableDevMode {

    fn process(&self, args: Vec<String>, flags: Vec<String>, value_flags: HashMap<String, String>) {

        // Heading
        cli_header("Enable Developer Mode");
        cli_send!("Please specify a directory where the code for your plugin projects will reside.\n\n");

        // Get dir
        let mut plugin_dir = String::new();
        loop {
            let default_dir = format!("{}/dev-plugins", CONFIG.general.libdir);
            let message = format!("Plugin Directory [{}]: ", default_dir);
            plugin_dir = cli_get_input(&message.as_str(), &default_dir.as_str());

            // Check if dir exists
            if Path::new(&plugin_dir).exists() {
                break;
            }

            // Prompt to create directory
            if !cli_confirm("The directory does not currently exist, would you like me to create it?") {
                continue;
            }

            // Create dir
            match fs::create_dir_all(&plugin_dir) {
                Ok(_) => { },
                Err(e) => {
                    error!("Unable to create directory, {}, error: {}", plugin_dir, e);
                    std::process::exit(1);
                }
            };
            break;
        }

        // Update config
        let mut config = CiceroServerConfig::new();
        config.general.plugin_dev_dir = plugin_dir.trim_end_matches(MAIN_SEPARATOR).to_string();
        config.save();

        // Restart apoolo
        let mut mgr = ProcessManager::new();
        mgr.stop("apollo");
        mgr.start("apollo", CONFIG.daemons.apollo.1 as i16, 1);

        // Success
        cli_send!("Successfully enabled developer mode.  The below directory will now be scanned each time Cicero boots for plugins, and will automatically compile any modifications detected.\n\n");
        cli_send!("    {}\n\n", plugin_dir);
    }

    fn help(&self) -> CliHelpScreen {

        let mut help = CliHelpScreen::new(
            "Enable Developer Mode",
            "cicero enable-dev-mode",
            "Enables developer mode and Cicero will begin dynamically loading any plugins under development."
        );

        help.add_example("cicero enable-dev-mode");
        help
    }

}


