
use std::collections::HashMap;
use falcon_cli::help::CliHelpScreen;
use falcon_cli::*;
use crate::llm::api;

#[derive(Default)]
pub struct LLMChat { }
impl CliCommand for LLMChat {

    fn process(&self, args: Vec<String>, flags: Vec<String>, value_flags: HashMap<String, String>) {

        cli_header("Chat with Cicero");
        cli_send!("Write what you wish to send to Cicero.  Please enter twice to send the message.\n\n");

        // Get input
        let mut input: Vec<String> = Vec::new();
        let mut prev_line_empty = false;
        loop {
            let line = cli_get_input("", "");
            if line.time().is_empty() && prev_line_empty {
                break;
            } else if line.trim().is_empty() {
                prev_line_empty = true;
            } else {
                prev_line_empty = false;
            }
            input.push(line.clone);

            // Send message
            let message = input.join("\n").to_string();
            let res = api.send_chat(&message).unwrap();

            println!("Cicero's Response: {}", res);
            cli_send!("\nYour Reply: ");
        }

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



