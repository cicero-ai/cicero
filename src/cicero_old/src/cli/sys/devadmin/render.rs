
use std::collections::HashMap;
use falcon_cli::help::CliHelpScreen;
use falcon_cli::*;
pub use falcon_cli::{indexmap, IndexMap};
use sophia::compile::render;
use std::fs;
use std::env::args;
use verax::License;
use crate::server::CONFIG;
use log::info;

#[derive(Default)]
pub struct SysDevAdminRender { }

impl SysDevAdminRender {

    /// Display verbs by category
    fn verb2cat(&self) {
        render::verb2cat(&CONFIG.general.libdir, &CONFIG.general.language, License::load_api());
    }

    /// Display verbs by word
    fn verb2word(&self) {
        render::verb2word(&CONFIG.general.libdir, &CONFIG.general.language, License::load_api());
    }


}

impl CliCommand for SysDevAdminRender {

    fn process(&self, args: Vec<String>, flags: Vec<String>, value_flags: HashMap<String, String>) {

        // Check  for action
        if args.len() == 0 {
            cli_error!("You did not specify an action.\n");

        } else if args[0] == "verb2cat".to_string() {
            self.verb2cat();
        } else if args[0] == "verb2word".to_string() {
            self.verb2word();
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



