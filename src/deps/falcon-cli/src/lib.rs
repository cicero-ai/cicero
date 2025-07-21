#![crate_type = "lib"]
//#[allow(unused_assignments)]
#![allow(clippy::borrowed_box)]
//
// Easily and efficiently develop high quality and fully featured CLI apps.  Supports the following:
//
// * Excellent and easily accessible functions to send wordwrapped text, send a header, get user input, get password, get new password (ie. enter twice to confirm), get y/n confirmation, display table layout ala mySQL prompt, display two column borderless table, and more.
// * Easily add routes to new CLI commands.  Simply define the impl of the CLI command, the command alias, shortcuts, and any flags that will contain values.
// * Built-in help screens for every CLI command
// * Utilizes the levenshtein algorithm to automatically identify and correct typos within command names.
// * Categorize commands to one or more levels for better organization.
//
//  For full usage details and code examples, please visit the [Readme file](https://github.com/mdizak/rust-aalcon-cli/).
//

use help::CliHelpScreen;
pub use indexmap::{indexmap, IndexMap};
pub use textwrap::Options as Textwrap_Options;
pub use textwrap::fill as Textwrap_Fill;
use router::CliRouter;
use rpassword::read_password;
use std::collections::HashMap;
pub use std::io;
pub use std::io::Write;
use zxcvbn::zxcvbn;

#[cfg(feature = "log")]
use log::{error};

pub mod help;
pub mod router;

pub trait CliCommand {
    fn process(&self, args: Vec<String>, flags: Vec<String>, value_flags: HashMap<String, String>);
    fn help(&self) -> CliHelpScreen;
}

// Execute the necessary CLI command based on arguments passed.  This function should be executed once all necessary outes have been defined.
pub fn cli_run(router: &CliRouter) {
    // Lookup route
    let (cmd, req) = router.lookup();

    // Process as needed
    if req.is_help {
        CliHelpScreen::render(cmd, &req.cmd_alias, &req.shortcuts);
    } else {
        cmd.process(req.args, req.flags, req.value_flags);
    }
}

// Display header.  Outputs given text with 30 line of dashes at the top and bottom to signify a header.
pub fn cli_header(text: &str) {
    println!("------------------------------");
    println!("-- {}", text);
    println!("------------------------------\n");
}

/// utput text wordwrapped to 70 characters per-line.
#[macro_export]
macro_rules! cli_send {
    ($text:expr) => {
        let wrapped_text = Textwrap_Fill($text, Textwrap_Options::new(75));
        println!("{}", wrapped_text);
        io::stdout().flush().unwrap();
    };
    ($text:expr, $( $arg:expr ),*) => {

        // Gather args
        let mut args = vec![];
        $( args.push($arg.to_string()); )*

        // Replace placeholders
        let mut text: String = $text.to_string();
        for arg in args {
            text = text.replacen("{}", arg.to_string().as_str(), 1);
        }

        // DIsplay text
        let wrapped_text = Textwrap_Fill(text.as_str(), Textwrap_Options::new(75));
        println!("{}", wrapped_text);
        io::stdout().flush().unwrap();
    };
}

/// Output wordwrapped text to stdout, and also add a log::info!() message.
#[macro_export]
macro_rules! cli_info {
    ($text:expr) => {

        #[cfg(feature = "log")]
        log::info!("{}", $text);

        let wrapped_text = Textwrap_Fill($text, Textwrap_Options::new(75));
        println!("{}", wrapped_text);
        io::stdout().flush().unwrap();
    };
    ($text:expr, $( $arg:expr ),*) => {

        // Gather args
        let mut args = vec![];
        $( args.push($arg.to_string()); )*

        // Replace placeholders
        let mut text: String = $text.to_string();
        for arg in args {
            text = text.replacen("{}", arg.to_string().as_str(), 1);
        }

        #[cfg(feature = "log")]
        log::info!("{}", text);

        // Display text
        let wrapped_text = Textwrap_Fill(text.as_str(), Textwrap_Options::new(75));
        println!("{}", wrapped_text);
        io::stdout().flush().unwrap();
    };
}

// Pass an IndexMap (similar to HashMap but remains ordered) and will return the option the user selects.  Will not return until user submits valid option value.
pub fn cli_get_option(question: &str, options: &IndexMap<String, String>) -> String {
    let message = format!("{}\r\n\r\n", question);
    cli_send!(&message);
    for (key, value) in options.iter() {
        let line = format!("    [{}] {}\r\n", key, value);
        cli_send!(&line);
    }
    cli_send!("\r\nSelect One: ");

    // Get user input
    let mut input: String;
    loop {
        input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();
        if !options.contains_key(input) {
            print!("\r\nInvalid option, try again: ");
            io::stdout().flush().unwrap();
        } else {
            break;
        }
    }

    input.trim().to_string()
}

// Get input from the user
pub fn cli_get_input(message: &str, default_value: &str) -> String {
    // Display message
    cli_send!(message);
    io::stdout().flush().unwrap();

    // Get user input
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let mut input = input.trim();

    // Default value, if needed
    if input.trim().is_empty() {
        input = default_value;
    }

    String::from(input)
}

// Request confirmation from the user
pub fn cli_confirm(message: &str) -> bool {
    // Send message
    let confirm_message = format!("{} (y/n): ", message);
    cli_send!(&confirm_message);

    // Get user input
    let mut _input = "".to_string();
    loop {
        _input = String::new();

        io::stdin()
            .read_line(&mut _input)
            .expect("Failed to read line");
        let _input = _input.trim().to_lowercase();

        if _input != "y" && _input != "n" {
            cli_send!("Invalid option, please try again.  Enter (y/n): ");
        } else {
            break;
        }
    }

    // Return
    let res_char = _input.chars().next().unwrap();

    res_char == 'y'
}

// Get a single password without the user's input being output to the terminal.
pub fn cli_get_password(message: &str) -> String {
    // Get message
    let password_message = if message.is_empty() {
        "Password: "
    } else {
        message
    };

    // Get password
    let mut _password = String::new();
    loop {
        cli_send!(password_message);
        _password = read_password().unwrap();

        if _password.is_empty() {
            cli_send!("You did not specify a password");
        } else {
            break;
        }
    }

    _password
}

// Get a new password that does both, ensures the user types it in twice to confirm and also checks for desired security strength.  The 'strength' parameter can be 0 - 4.
pub fn cli_get_new_password(req_strength: u8) -> String {
    // Initialize
    let mut _password = String::new();
    let mut _confirm_password = String::new();

    // Get new password
    loop {
        cli_send!("Desired Password: ");
        _password = read_password().unwrap();

        if _password.is_empty() {
            cli_send!("You did not specify a password");
            continue;
        }

        // Check strength
        let strength = zxcvbn(&_password, &[]).unwrap();
        if strength.score() < req_strength {
            cli_send!("Password is not strong enough.  Please try again.\n\n");
            continue;
        }

        // Confirm password
        cli_send!("Confirm Password: ");
        _confirm_password = read_password().unwrap();
        if _password != _confirm_password {
            cli_send!("Passwords do not match, please try again.\n\n");
            continue;
        }
        break;
    }

    _password
}

// Display a tabular output similar to any SQL database prompt's output
pub fn cli_display_table(columns: Vec<&str>, rows: Vec<Vec<&str>>) {
    // Return if no rows
    if rows.is_empty() {
        cli_send!("No rows to display.\n\n");
        return;
    }

    // nitialize sizes
    let mut sizes: HashMap<&str, usize> = HashMap::new();
    for col in columns.as_slice() {
        sizes.insert(col, 0);
    }

    // Get sizes of columns
    for _row in rows.clone() {
        for col in columns.as_slice() {
            let num: usize = col.len();
            if num > sizes[col] {
                sizes.insert(col, num + 3);
            }
        }
    }

    // Initialize header variables
    let mut header = String::from("+");
    let mut col_header = String::from("|");

    // Print column headers
    for col in columns.clone() {
        let padded_col = format!("{}{}", col, " ".repeat(sizes[col] - col.len()));
        header = header + "-".repeat(sizes[col] + 1).as_str() + "+";
        col_header += format!(" {}|", padded_col).as_str();
    }
    println!("{}\n{}\n{}", header, col_header, header);

    // Display the rows
    for row in rows {
        // Go through values
        let mut line = String::from("|");
        for (i, val) in row.into_iter().enumerate() {
            let padded_val = format!(" {}{}", val, " ".repeat(sizes[columns[i]] - val.len()));
            line += format!("{}|", padded_val).as_str();
        }
        println!("{}", line);
    }
    println!("{}\n", header);
}

// Display a two column array with proper spacing.  Mainly used for the help() function to display available parameters and flags.
pub fn cli_display_array(rows: &IndexMap<String, String>) {
    // Get max left column size
    let mut size = 0;
    for key in rows.keys() {
        if key.len() + 8 > size {
            size = key.len() + 8;
        }
    }
    let indent = " ".repeat(size);
    let indent_size = size - 4;

    // Go through rows
    for (key, value) in rows {
        let left_col = format!("    {}{}", key, " ".repeat(indent_size - key.len()));
        let options = textwrap::Options::new(75)
            .initial_indent(&left_col)
            .subsequent_indent(&indent);
        let line = textwrap::fill(value, &options);
        println!("{}", line);
    }
    cli_send!("");
}

// Give error message to stdout, plus log via log::error!()
#[macro_export]
macro_rules! cli_error {
    ($text:expr) => {

        #[cfg(feature = "log")]
        error!("{}", $text);

        let wrapped_text = Textwrap_Fill(format!("ERROR: {}", $text).as_str(), Textwrap_Options::new(75));
        println!("{}\r\n", wrapped_text);
        io::stdout().flush().unwrap();
    };
    ($text:expr, $( $arg:expr ),*) => {

        // Gather args
        let mut args = vec![];
        $( args.push($arg.to_string()); )*

        // Replace placeholders
        let mut text: String = $text.to_string();
        for arg in args {
            text = text.replacen("{}", arg.to_string().as_str(), 1);
        }

        #[cfg(feature = "log")]
        error!("{}", text);

        // Display text
        let wrapped_text = Textwrap_Fill(format!("ERROR: {}", text).as_str(), Textwrap_Options::new(75));
        println!("{}\r\n", wrapped_text);
        io::stdout().flush().unwrap();
    };
}


// Output success message that displays a vector of filenames or anything else indented below the message.
pub fn cli_success (message: &str, indented_lines: Vec<&str>) {
    cli_send!(&message);
    cli_send!("\r\n");
    for line in indented_lines {
        println!("    {}", line);
    }
    cli_send!("\r\n");
}


// Clear all lines within terminal and revert to blank terminal screen.
pub fn cli_clear_screen() {
    print!("\x1B[2J");
}
