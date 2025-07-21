
use std::collections::HashMap;
use falcon_cli::help::CliHelpScreen;
use falcon_cli::*;
use crate::client::ClientUser;
use crate::server::security::forge;
use crate::client::CiceroClientConfig;

use crate::utils::api_client;
use log::error;

#[derive(Default)]
pub struct ProfileCreate { }
impl CliCommand for ProfileCreate {

    fn process(&self, args: Vec<String>, flags: Vec<String>, value_flags: HashMap<String, String>) {

        cli_header("Create Profile");
        cli_send!("Need a few short details to get a new Cicero profile setup for you.\n\n");

        // Get input
        let name = cli_get_input("Your Name: ", "");
        let password = cli_get_new_password(0);
        let email = cli_get_input("Optional E-Mail address: ", "");

        // Auto login?
        println!("");
        cli_send!("You may optionally enable auto-login, meaning you will not be prompted for a password upon opening Cicero, but naturally lowers your security.\n\n");
        let auto_login = cli_confirm("Enable auto-login?");

        // Create
        let user = match ClientUser::create(&name, &password, &email, &auto_login) {
            Ok(r) => r,
            Err(e) => {
                cli_error!("Unable to create user, error: {}", e.to_string());
                std::process::exit(1);
            }
        };

        // Add local user
        let norm_password = forge::normalize_password(&password);
        CiceroClientConfig::add_local_user(&user, Some(norm_password.clone()));

        // Get bip39 words
        let words = forge::get_bip39_words(&password.as_str());
        println!("");

        // Recovery pass phrase
        cli_header("Recovery Passphrase");
        cli_send!("Cicero greatly values your personal privacy and security.  By default all communication is securely encrypted, and below is a recovery passphrase in case of a lost password.\n\n");
    for batch in words.chunks(8) {
            println!("    {}", batch.join(" ").to_string());
        }
        println!("");
        cli_send!("Write down the above words and save them somewhere safe, as they will allow you to recover your account in case of lost password.  Once done, press enter to complete setup.");
        cli_get_input("", "");

        // Success message
        if !flags.contains(&"q".to_string()) {
            cli_send!("Congratulations, the new user '{}' has been successfully created and is ready for use.  To begin a chat with Cicero, simply run the command:\n\n");
            cli_send!("    cicero chat\n");
        }
    }

    fn help(&self) -> CliHelpScreen {

        let mut help = CliHelpScreen::new(
            "Create New Profile", 
            "cicero profile create",
            "Create a new profile."
        );
        help.add_example("cicero profile create");

        help
    }

}



