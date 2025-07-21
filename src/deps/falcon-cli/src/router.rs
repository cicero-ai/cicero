use crate::help::CliHelpScreen;
use crate::CliCommand;
use crate::*;
use std::collections::HashMap;
use std::env;
use strsim::levenshtein;

pub struct CliRouter {
    pub commands: HashMap<String, Box<dyn CliCommand>>,
    pub shortcuts: HashMap<String, String>,
    pub value_flags: HashMap<String, Vec<String>>,
    pub categories: HashMap<String, (String, String)>,
}

pub struct CliRequest {
    pub cmd_alias: String,
    pub is_help: bool,
    pub args: Vec<String>,
    pub flags: Vec<String>,
    pub value_flags: HashMap<String, String>,
    pub shortcuts: Vec<String>,
}

impl Default for CliRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl CliRouter {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
            shortcuts: HashMap::new(),
            value_flags: HashMap::new(),
            categories: HashMap::new(),
        }
    }

    /// Link a struct / impl that inherits the CliRouter trait to a command name.  Takes three arguments,
    /// the full name of the command, a vector of available shortcuts, and a vector of
    /// long-form flags (prefixed with dashes (--)) for which a value is expected.
    pub fn add<T: CliCommand + Default + 'static>(
        &mut self,
        alias: &str,
        shortcuts: Vec<&str>,
        value_flags: Vec<&str>,
    ) {
        // Add to list of commands
        let cmd = Box::<T>::default();
        self.commands.insert(alias.to_string(), cmd);

        // Add shortcuts
        for shortcut in shortcuts {
            self.shortcuts
                .insert(shortcut.to_string(), alias.to_string());
        }

        // Add value flags
        let flags: Vec<String> = value_flags.iter().map(|s| s.to_string()).collect();
        self.value_flags.insert(alias.to_string(), flags);
    }

    /// Taking arguments passed via command line into account,  checks all routes that were added and
    /// tries to determine the correct impl to execute.  This function is automatically
    /// executed by the cli_run() function and should never be manually executed.
    pub fn lookup(&self) -> (&Box<dyn CliCommand>, CliRequest) {
        // Get args
        let mut cmdargs: Vec<String> = env::args().collect();
        cmdargs.remove(0);
        if cmdargs.is_empty() {
            cli_error!("You did not specify a command to run.  Please specify a command or use 'help' or '-h' to view a list of all available commands.");
            std::process::exit(0);
        }

        // Blank variables
        let mut extra_args: Vec<String> = Vec::new();
        let mut cmd_alias = String::new();

        // Check if help
        let mut is_help: bool = false;
        if cmdargs.len() > 0 && (cmdargs[0] == "help" || cmdargs[0] == "-h") {
            is_help = true;
            cmdargs.remove(0);
        }

        // Check for help index
        if is_help && cmdargs.is_empty() {
            CliHelpScreen::render_index(self);
        }

        // Check routing table for command
        loop {
            // Check for zero cmdargs
            if cmdargs.is_empty() && is_help {
                break;
            } else if cmdargs.is_empty() {
                cli_error!("No command exists with that name.  Use either 'help' or the -h flag to view a list of all available commands.");
                std::process::exit(0);
            }
            let alias = cmdargs.iter().map(|v| v.to_string()).filter(|a| !a.starts_with("-")).collect::<Vec<String>>().join(" ").to_string();

            if self.commands.contains_key(&alias) {
                cmd_alias = alias;
                break;
            } else if self.shortcuts.contains_key(&alias) {
                cmd_alias = self.shortcuts.get(&alias).unwrap().to_string();
                break;
            } else if is_help && self.categories.contains_key(&alias) {
                CliHelpScreen::render_category(self, &alias);
            } else if let Some(found_cmd) = self.lookup_similar(&alias) {
                let confirm_msg = format!("No command with that name exists, but a similar command with the name '{}' does exist.  Is this the command you wish to run?", found_cmd);
                if cli_confirm(&confirm_msg) {
                    cmd_alias = found_cmd.to_string();
                    break;
                }
            }
            extra_args.insert(0, cmdargs.pop().unwrap());
        }

        // Set variables
        let mut args: Vec<String> = Vec::new();
        let mut flags: Vec<String> = Vec::new();
        let mut value_flags: HashMap<String, String> = HashMap::new();
        let flag_values = self.value_flags.get(&cmd_alias).unwrap_or(&Vec::new()).clone();

        // Get flags
        while !extra_args.is_empty() {
            let chk_arg = extra_args[0].to_string();
            extra_args.remove(0);

            if chk_arg.starts_with("--") {
                let arg = chk_arg.trim_start_matches('-').to_string();
                if flag_values.contains(&arg) {
                    value_flags.insert(arg, extra_args[0].to_string());
                    extra_args.remove(0);
                } else {
                    flags.push(arg);
                }
            } else if chk_arg.starts_with('-') {
                let arg = chk_arg.trim_start_matches('-').to_string();
                for c in arg.chars() {
                    flags.push(c.to_string());
                }
            } else {
                args.push(chk_arg);
            }
        }

        // Get all shortcuts
        let shortcuts: Vec<String> = self
            .shortcuts
            .iter()
            .filter_map(|(key, value)| {
                if *value == cmd_alias {
                    Some(key.to_string())
                } else {
                    None
                }
            })
            .collect();

        let cmd = match self.commands.get(&cmd_alias) {
            Some(r) => r,
            None => {
                CliHelpScreen::render_index(self);
                std::process::exit(0);
            }
        };

        let req = CliRequest {
            cmd_alias,
            is_help,
            args,
            flags,
            value_flags,
            shortcuts,
        };

        (cmd, req)
    }

    /// Never needs to be manually executed, and is used when a full match for the command name can not
    /// be found.  Uses the levenshtein to see if any commands closely resemble the
    /// command name given in case of typo.
    fn lookup_similar(&self, chk_cmd: &String) -> Option<&String> {
        let mut distance = 4;
        let mut res = None;

        for cmd in self.commands.keys() {
            let num = levenshtein(chk_cmd, cmd);
            if num < distance {
                distance = num;
                res = Some(cmd)
            }
        }

        res
    }

    /// Add a new top level category that contains CLI commands.  Used for organization and to display proper help screens.
    pub fn add_category(&mut self, alias: &str, name: &str, description: &str) {
        self.categories.insert(
            alias.to_string(),
            (name.to_string(), description.to_string()),
        );
    }
}
