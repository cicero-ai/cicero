
use std::collections::HashMap;
use crate::error::Error;
use falcon_cli::help::CliHelpScreen;
use falcon_cli::*;
use crate::client::ClientUser;
use crate::server::security::forge;
use crate::utils::{sys, ProcessManager};
use crate::CLIENT_CONFIG;
use uuid::Uuid;
use std::path::Path;
use std::fs;
use webbrowser;
use falcon_cli::{indexmap, IndexMap};
use hex;

#[derive(Default)]
pub struct ProfileLogin { }
impl CliCommand for ProfileLogin {

    fn process(&self, args: Vec<String>, flags: Vec<String>, value_flags: HashMap<String, String>) {

        // Select user, if needed
        let uuid: Uuid = self.get_uuid();

        let mut password = String::new();
    let mut norm_password: [u8; 32] = [0; 32];

        // Check for autologin file
        let autologin_file = format!("{}/clients/.{}.login", sys::get_datadir(), uuid.to_string());
        if Path::new(&autologin_file).exists() {
            password = fs::read_to_string(&autologin_file).unwrap();
            norm_password.copy_from_slice(&hex::decode(&password).unwrap());

        // Check password file, if defined and exists
        } else if value_flags.contains_key("password-file") {
            let pass_file = value_flags.get("password-file").unwrap();
            if Path::new(&pass_file).exists() {
                password = fs::read_to_string(&pass_file).expect("Unable to read from temporary ~/tmp/.login file").trim().to_string();
                norm_password = forge::normalize_password(&password.as_str());
            }

        // Prompt for password
        } else {
            password = cli_get_password("Password: ");
            norm_password = forge::normalize_password(&password);
        }

        // Load user
        let mut user: Option<ClientUser> = None;
        loop {

            if let Ok(res) = ClientUser::load(&uuid, &norm_password) {
                user = Some(res);
                break;
            }

            cli_error!("Invalid password, please try again.\n");
            password = cli_get_password("Password: ");
            norm_password = forge::normalize_password(&password);
        }

        // Save temporary password file
        let filename = format!("{}/tmp/.login", sys::get_datadir());
        sys::prepare_parent_dir(&filename);
        fs::write(&filename, format!("{}\n{}", uuid.to_string(), hex::encode(&norm_password)))
            .expect("Unable to write to temporary file within ~/tmp/ directory of local datadir.");

        // Start echo server
        let mut manager = ProcessManager::new();
        manager.start("echo", CLIENT_CONFIG.daemons.echo.1 as i16, 1);

        // Open web broser
        let url = format!("http://{}:{}/account", CLIENT_CONFIG.daemons.echo.0, CLIENT_CONFIG.daemons.echo.1);
        if flags.contains(&"c".to_string()) {
            match webbrowser::open(&url) {
                Ok(_) => { },
                Err(_) => { }
            };


            // Success message
            println!("");
            cli_header("Login Successful");
            cli_send!("You have been successfully logged into Cicero!  Your web browser should have already opened to your dashboard, but if not, simply open your web browser and visit the URL:\n\n");
            cli_send!("    {}\n\n", url);
            cli_send!("Alternatively, at any time you may open a new chat with Cicero by running the command:\n\n");
            cli_send!("    cicero chat\n\n");
        } else {
            println!("");
            cli_header("Login Successful");
            cli_send!("You have been successfully logged into Cicero!  You may view the following URL in your browser to chat with Cicero:\n\n");
            cli_send!("    {}\n\n", url);
            cli_send!("Alternatively, at any time you may open a new chat with Cicero by running the command:\n\n");
            cli_send!("    cicero chat\n\n");
            cli_send!("If you would like the browser automatically opened upon login, simply add the -c option when running the login command.\n\n");
        }


    }

    fn help(&self) -> CliHelpScreen {

        let mut help = CliHelpScreen::new(
            "Login",
            "cicero login [--password-file <PASSWORD_FILE>",
            "Login to Cicero with a saved profile on this machine."
        );

        help.add_flag("--password-file", "If present, the password contained within this file will be used instead of prompting the user for a password.");
        help.add_example("cicero login");

        help
    }

}

impl ProfileLogin {

    /// Get uuid
    fn get_uuid(&self) -> Uuid {

        // Check for one user
        if CLIENT_CONFIG.local_users.len() == 1 {
            return CLIENT_CONFIG.local_users.iter().next().unwrap().0.clone();
        } else if CLIENT_CONFIG.local_users.keys().len() == 0 {
            cli_error!("There are no saved profiles on this machine.  Please first create a profile by running the command:\n\n");
            println!("    ./cicero profile create\n\n");
            std::process::exit(0);
        }

        // List users and ask to choose
        cli_send!("Select profile to login with:\n\n");
        let (mut x, mut users, mut options) = (1, Vec::new(), IndexMap::new());
        for (uuid, name) in CLIENT_CONFIG.local_users.iter() {
            options.insert(x.to_string(), name.clone());
            users.push(uuid.clone());
            x += 1;
        }
        let sel_str = cli_get_option("Select Profile: ", &options);
        let sel = sel_str.parse::<usize>().unwrap() - 1;
        users[sel].clone()
    }

}


