
# Falcon CLI - Usage

There's a few things to note:

* Every CLI command is a separate struct / impl that inherits the CliCommand trait which requires two methods -- process() and help().
* You must link each struct / impl via the CliRouter::add() method for the CLI command to work, as exampled below.
* Every impl that represents a CLI command must have a help() method that returns an instance of the CliHelpScreen struct. 
* Categorizing commands into groups for better organization is fully supported, including multiple levels of categories.
* the cli_send!() and cli_error!() macros work exactly the same as print!(), accept an unlimited number of placeholders and arguments, but also wordwrap the text to 75 characters.

#### CLI Command struct / impl

Below is an example struct / impl that inherits the CliCommand trait.

~~~

    use std::collections::HashMap;
    use falcon_cli::help::CliHelpScreen;
    use falcon_cli::*;

    #[derive(Default)]
    pub struct CreateDomain {}

    impl CliCommand for CreateDomain {

        fn process(&self, args: Vec<String>, flags: Vec<String>, value_flags: HashMap<String, String>) {

            // Cecks
            if args.len() == 0 {
                cli_error!("You did not specify a domain name to create.");
            } else if !value_flags.contains_key("ip-address") {
                cli_error!("You did not specify an '--ip-address' flag specifying the IP address to assign the domain to.");
            }
            let ip_address = value_flags.get("ip-address").unwrap();

            // Confirn, unless -n flag is present
            if !flags.contains(&"n".to_string()) {
                let confirm_msg = format!("Are you sure you wish to create the domain {} on the IP address {}? ", args[0], ip_address);
                if !cli_confirm(&confirm_msg) {
                    cli_send~("Aborting.\r\n\r\n");
                    return;
                }
            }

            // Create domain here

            // Add to Nginx, if needed
            if flags.contains(&"nginx".to_string()) {
                // Add to Nginx here
            }

            // Success
            cli_success("Successfully created the domain:", vec![&args[0]]);
        }

        fn help(&self) -> CliHelpScreen {

            let mut help = CliHelpScreen::new("Create Domain Name", "mycli domain create <DOMAIN_NAME> --ip-address <IP_ADDR> [-n] [--nginx]", "Create a new domain name, and optionally add it to Nginx configuration.");

            help.add_param("DOMAIN_NAME", "The domain name to create.");

            help.add_flag("--ip-address", "IP address to assign the new domain name to.");
            help.add_flag("--nginx", "Optional, and if present will add domain name to Nginx configuration.");
            help.add_flag("-n", "Operation, non-interactive mode.  If present, will not ask to confirm creation.");

            help.add_example("mycli domain create some-domain.com --ip-address 24.162.84.178 --nginx -n");
            help
        }

    }
~~~


#### Adding Routes

Once you have the necessary struct / impl in place, you need to link them to the desired command by adding routes.  For example:

~~~
    use falcon_cli::*;
    use falcon_cli::router::CliRouter;
    use crate::create_domain::CreateDomain;

    pub mod create_domain;

    fn main() {

        // Send header
        cli_header("Example Falcon CLI App");

        // Add route
        let mut router = CliRouter::new();
        router.add::<CreateDomain>("domain create", vec!["dom c"], vec!["ip-address"]); 

        // Execute CLI
        cli_run(&router);
    }
~~~

Now you may run the program via cargo with a command such as:

`cargo run -- domain create example.com --ip-address 24.68.0.126 --nginx`

This will execute the process() method within the CreateDomain impl, which accepts three parameters:

( Vector of all additional arguments passed via CLI not including the command name itself or any flags.
* Vector of all flags without values, either short flags prefixed with a single dash (-), or long form flags prefixed with a double dash (--).
* Upon adding the route, the third parameter was a vector of all long-form flags (with two dashes) that will contain a value.  We specified `--ip-address` for this, hence the third parameter passed is a HashMap of all flags that contain values.

You may also view the help screen with the command:

`cargo run -- help domain create`

This will display a nicely formatted help screen that contains all details contained within the help() method of the CreateDomain impl.


#### Add Categories

If you have a good number of CLI commands you may organize them into categories (eg. "account ALIAS", "domain ALIAS", etc.).  The 
command names are then seperated by a space (eg. "account create", "account list", etc.).  You may also use multiple category levels so for example, 
you may create a categoryatabase at "sys smtp" and have a CLI command at "sys smtp add".

Categories can be added via the router.add_category() method, for example:

    use falcon_cli::router::CLiROuter;
~~~
    fn main() {

    let router = CliRouter::new();
        router.add_category("account", "User Accounts", "Create, manage and delete user accounts within the system.");
        router.add_category("domain", "Domain Names", "Create, manage and delete domain names on the server.");

        router.add::<CreateDomain>("domain create", vec!["dom c"], vec!["ip-address"]); 

        // Execute
        cli_run(&router);
    }
~~~

Now when you run the command:

> `cargo run -- help`

Instead of listing all availble commands, it will now list all available categories.  You may then view all commands available within a category with for example:

> `cargo run -- help domain`

Categories have no functionality purpose except for organization of help screens as exampled above.



