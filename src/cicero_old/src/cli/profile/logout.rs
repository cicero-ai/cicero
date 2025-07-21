
use std::collections::HashMap;
use falcon_cli::help::CliHelpScreen;
use falcon_cli::*;
use crate::client::ClientUser;
use crate::server::security::forge;
use crate::client::CiceroClientConfig;
use crate::utils::ProcessManager;

use crate::utils::api_client;
use log::error;

#[derive(Default)]
pub struct ProfileLogout { }
impl CliCommand for ProfileLogout {

    fn process(&self, args: Vec<String>, flags: Vec<String>, value_flags: HashMap<String, String>) {

        // Stop server
        let mut manager = ProcessManager::new();
        manager.stop("echo");

        // Save config with no current_uuid
        let mut config = CiceroClientConfig::new();
        config.current_uuid = None;
        config.save();

        // Sucecss mesage
        cli_send!("Successfully logged out of Cicero.\n\n");
    }

    fn help(&self) -> CliHelpScreen {

        let mut help = CliHelpScreen::new(
            "Logout",
            "cicero logout",
            "Logout of Cicero"
        );
        help.add_example("cicero logout");

        help
    }

}



