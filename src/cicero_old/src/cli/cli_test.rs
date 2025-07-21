
use std::collections::HashMap;
use falcon_cli::help::CliHelpScreen;
use falcon_cli::*;
use crate::test;


#[derive(Default)]
pub struct CliTest { }


impl CliCommand for CliTest {

    fn process(&self, args: Vec<String>, flags: Vec<String>, value_flags: HashMap<String, String>) {
        crate::test::test();
        std::process::exit(0);
    }

    fn help(&self) -> CliHelpScreen {

        let mut help = CliHelpScreen::new(
            "Find Domain Name", 
            "nexus utils find-domain",
            "Come up with, and search for an available and un-registered domain name."
        );
        help.add_example("nexus utils find-domain");

        help
    }

}



