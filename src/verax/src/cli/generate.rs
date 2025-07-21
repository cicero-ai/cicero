
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use falcon_cli::help::CliHelpScreen;
use falcon_cli::*;
use verax::License;
use crate::hq::CiceroHQ;
use crate::packager::Packager;
use crate::{tools, CONFIG};
use bincode;
use std::fs;
use hex;

#[derive(Serialize, Deserialize)]
pub struct LicenseApiResponse {
    pub uuid: String,
    pub license_id: String,
    pub encryption_key: String
}
#[derive(Default)]
pub struct CliGenerate { }

impl CliCommand for CliGenerate {

    fn process(&self, args: Vec<String>, flags: Vec<String>, value_flags: HashMap<String, String>) {

        // Get info
        cli_header("Generate License");
        let username = cli_get_input("Username: ", "");
        let product = cli_get_input("Product [sophia]: ", "sophia");
        let req_body = format!("username={}&product={}", username, product);

        // Send http request
        let hq = CiceroHQ::new();
        let res = hq.send::<LicenseApiResponse>(&req_body);

        // Get encryption key
        let key_bytes = hex::decode(&res.encryption_key).unwrap();
        let encryption_key: [u8; 32] = key_bytes.try_into().unwrap();

        // Create new license
        let mut license = License::new(&res.license_id, &product, &username);
        license.encryption_key = encryption_key;

        #[cfg(feature="local")] {
            let local_file = format!("{}/local/license.dat", CONFIG.datadir);
            license.save_local(&local_file);
        }

        // Release
        let packager = Packager::new();
        packager.release(&license);

        cli_send!("Successfully completed release:\n\n");
        cli_send!("    License ID: {}\n", license.license_id);
        cli_send!("    Product: {}\n\n", license.product);
            cli_send!("All files located in the release directory data/release\n");
    }

    fn help(&self) -> CliHelpScreen {

        let mut help = CliHelpScreen::new(
            "Start Sophia Daemon",
            "sophia start",
            "Starts the Sophia daemon / RPC server, and keeps vocabulary database loaded into persistent memory."
        );

        help.add_example("sophia start");
        help
    }

}



