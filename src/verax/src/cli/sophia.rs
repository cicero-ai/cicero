

use std::collections::HashMap;
use falcon_cli::help::CliHelpScreen;
use falcon_cli::*;
use std::fs;
use std::env::args;
use sophia::Sophia;
use sophia::compile::database;
use verax::License;
use crate::config::Config;


pub struct CliSophia { 
    config: Config
}

impl CliSophia {

    // Compile vocab database
    fn compile(&self) {
        cli_send!("Compiling vocab, this may takea  few minutes...\n");




        let mut vocab = database::compile(&self.config.vocab_dir.as_str());

        // Save vocab file
        let filename = format!("{}/sophia/en.dat", self.config.datadir);
        fs::write(&filename, bincode::serialize(&vocab).unwrap()).unwrap();

        cli_send!("Successfully saved vocab file at, {}", filename);
    }

}


impl CliCommand for CliSophia {

    fn process(&self, args: Vec<String>, flags: Vec<String>, value_flags: HashMap<String, String>) {

        // Compile
        if args[0] == "compile".to_string() {
            self.compile();
        } else {
            cli_error!("Nothing to do");
        }

    }

    fn help(&self) -> CliHelpScreen {

        let mut help = CliHelpScreen::new(
            "Compile Vocabulary", 
            "cicero compile-vocab",
            "Compiles the vocabulary file from plain text vocab lists -- for maintainers of Cicero only."
        );

        help.add_example("cicero compile-vocab");
        help
    }

}

impl Default for CliSophia {
    fn default() -> CliSophia {
        CliSophia {
            config: crate::config::load()
        }
    }
}



