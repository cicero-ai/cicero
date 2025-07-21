
use falcon_cli::*;
use dirs;
use which::which;
use std::collections::HashMap;
use std::net::TcpStream;
use uuid::Uuid;
use crate::utils::random;
use crate::llm::models::ModelSize;
use crate::llm::api::api_selector;
use crate::llm::chat::ChatRouter;
use crate::server::config::CiceroServerConfig;
use crate::client::CiceroClientConfig;
use crate::utils::{sys, ProcessManager};
use std::path::{Path, MAIN_SEPARATOR};
use std::fs;
use falcon_cli::{indexmap, IndexMap};
use log::{warn, error};
use super::{Vault, HardwareProfile, CiceroHQ, ollama};
use super::hq::ConversationModel;

// Run setup
pub fn run() -> bool {

    /// Header
    cli_header("Cicero Setup");
    cli_send!("Welcome to Cicero, your new AI home assistant!  Let's quickly get you up and running.\n\n");

        // Server host
        cli_send!("If Cicero is already running, enter its host / IP address below.  Otherwise, or if you are unsure, press enter to indicate this is a new installation.\n\n");
        let server_host = cli_get_input("Server Host: ", "").trim().to_string();
        if server_host.is_empty() {
        first_time();
        return true;
        }
    let server_port = cli_get_input("Server Port [7511]: ", "7511");

    false

}

/// First time setup
fn first_time() {

    // Initial checks
    initial_checks();

    // Create datadir
    let datadir = create_datadir();

    // Create server config
    let uuid = Uuid::new_v4();
    let (mut config, size, conversation_models) = initialize_server_config(&uuid);

    // Select conversation model
    let mut router = select_conversation_model(&size, &conversation_models, &mut config);

    // Configure API client
    configure_api_client(&size, &mut router, &mut config);

    /// Ask to help Cicero development
    ask_help_cicero(&mut config);

    // Create vault
    let mut vault = create_vault(&uuid);

    // Save config
    config.save();

    // Success message
    success_message(&config, &mut vault);
}

/// Initial checks
fn initial_checks() {

    // Check is pollo already running
    match TcpStream::connect("127.0.0.1:7511") {
        Ok(_) => {
            error!("There is a server running on port 7511 indicating Cicero is already running on this machine.  Please close the process and try again.");
            std::process::exit(1);
        },
        Err(_) => { }
    };

    // Check is pollo already running
    match TcpStream::connect("127.0.0.1:5833") {
        Ok(_) => {
            error!("There is a server running on port 5833 indicating Cicero is already running on this machine.  Please close the process and try again.");
            std::process::exit(1);
        },
        Err(_) => { }
    };

}

/// Create data directory
fn create_datadir() -> String {

    // Get datadir
    let datadir = sys::get_datadir();

    // Create sub-directories
    for subdir in vec!["config", "manager", "profiles", "tmp"] {
        let dirname = format!("{}/{}", datadir, subdir);
        if Path::new(&dirname).exists() {
            continue;
        }

        // Create dir, if needed
        match fs::create_dir_all(&dirname) {
            Ok(_) => { },
            Err(e) => {
                error!("Unable to create local data directory, {}.  Error: {}", dirname, e);
                std::process::exit(1);
            }
        };
    }

    datadir
}

/// Create server config
fn initialize_server_config(uuid: &Uuid) -> (CiceroServerConfig, ModelSize, Vec<ConversationModel>) {

    // Get hardware profile
    let hw_profile = HardwareProfile::new();
    let size_str = serde_json::to_string(&hw_profile.llm_size).unwrap().to_lowercase().trim_end_matches('"').trim_start_matches('"').to_string();

    // Header
    println!("");
    cli_header("Initializing Cicero");
    cli_send!("This machine is running {}, setting up with a Cicero-{} configuration.  You may change this later.\n\n", hw_profile, size_str);

    // Get default models
    let hq = CiceroHQ::new();
    let (mut config, conversation_models) = hq.get_default_models(&hw_profile.llm_size, &uuid);

    // Get libdir
    config.general.libdir = discover_libdir();

    // Get network mode
    let (mode, apollo_host) = get_network_mode();
    config.general.network_mode = mode.clone();
    config.daemons.apollo.0 = apollo_host.clone();

    (config, hw_profile.llm_size, conversation_models)
}

/// Discover the l libbir
fn discover_libdir() -> String {

    // Check for dir
    if let Some(libdir) = sys::get_libdir() {
        return libdir;
    }

    cli_send!("Unable to determine the libdir.  This is where Cicero was downloaded / unpacked to and contains the file /nlu/en.dat.  Please enter the directory's location:\n\n");
    let mut library_dir = String::new();
    loop {

        let libdir = cli_get_input("Path to lib Directory: ", "");
        let filename = format!("{}/nlu/en.dat", libdir.trim_end_matches(MAIN_SEPARATOR).to_string());
        if Path::new(&filename).exists() {
            library_dir = libdir.trim_end_matches(MAIN_SEPARATOR).to_string();
            break;
        }
        println!("Error: Invalid lib directory, it must contain the /nlu/en.dat file.  Please try again.\n");
    }

    library_dir
}

/// Get network mode and Apollo host
fn get_network_mode() -> (String, String) {

    // Start options
    let mut options = IndexMap::new();
    options.insert("1".to_string(), "Local Machine - This machine only (127.0.0.1".to_string());

    // Local network address
    if let Some(local_ip) = sys::determine_ip_address("192.168.0.1:80") {
        options.insert("2".to_string(), format!("Local Network - Anyone on this local network / router ({})", local_ip));
    } else {
        warn!("Unable to determine local router IP address.");
    }
    options.insert("3".to_string(), "Public Internet - Connect from anywhere (0.0.0.0)".to_string());

    // Get network mode
    cli_header("Network Mode");
    let mode = cli_get_option("Where will people be connecting to and using Cicero from? ", &options);

    let (mut network_mode, mut ipaddr) = ("local", "127.0.0.1".to_string());
    if mode == "2".to_string() {
        network_mode = "lan";
        ipaddr = sys::determine_ip_address("192.168.0.1:80").unwrap();
    } else if mode == "3".to_string() {
        network_mode = "internet";
        ipaddr = "0.0.0.0".to_string();
        println!("");
        warn!("NOTE: Please ensure your router / firewall is properly configured to allow incoming connections on ports 7511 and 5833.\n");
    }

    (network_mode.to_string(), ipaddr)
}

// Select conversation model
fn select_conversation_model(size: &ModelSize, conversation_models: &Vec<ConversationModel>, config: &mut CiceroServerConfig) -> ChatRouter {

    let mut router = ChatRouter::new();
    cli_header("Select Conversation Model");
    cli_send!("Our sincerest apologies, but due to a temporary technical issue interfacing with larger, modern LLMs in Rust, we currently recommend using Ollama (https://ollama.com/) for generating conversational replies. We are diligently working to resolve this issue and will release a new version of Cicero shortly without the Ollama dependency.\n\n");
    cli_send!("Please note, this does not affect the understanding or functionality of Cicero whatsoever, as the NLU and task engines are internal.  Although the LLM is used as a fallback, it's mainly used to generate conversational replies, nothing more.\n\n");

    // Setup ollama
    let router = ollama::setup(&size, &conversation_models, config);
    router
}

// Select conversation model
fn select_conversation_model_old(size: &ModelSize, conversation_models: &Vec<String>) -> ChatRouter {

    // Check model size
    let mut router = ChatRouter::default();
    if *size == ModelSize::Tiny || *size == ModelSize::Small {
        //return router;
    }

    // Header
    cli_header("Select Conversation model");
    cli_send!("Great, your machine meets the necessary requirements to host the conversational model locally!  Select a model from below for Cicero to use:\n\n");

    // Create index map
    let mut x = 1;
    let mut options = IndexMap::new();
    for name in conversation_models {
        options.insert(x.to_string(), name.to_string());
        x += 1;
    }
    options.insert(x.to_string(), "Remote Python API (see documentation)".to_string());

    // Add none option
    x += 1;
    options.insert(x.to_string(), "None, integrate with external API (OpenAI, Claude, etc.)".to_string());

    // Get option
    let chat_model = cli_get_option("Select Model: ", &options);
    if chat_model == x.to_string() {
        return router;
    } else if chat_model == (x - 1).to_string() {
        router.default_model = "remote_python_api".to_string();
        return router;
    }

    let index = chat_model.parse::<usize>().unwrap() - 1;
    router.default_model = conversation_models[index].to_string();
    router
}

/// Configure API client
fn configure_api_client(size: &ModelSize, router: &mut ChatRouter, config: &mut CiceroServerConfig) {

    let (mut is_required, mut show_remote_python) = (false, false);
    if router.default_model.is_empty() {
        cli_header("API Client");
        cli_send!("Due to hardware constraints, you must utilize an external API to converse with Cicero to enjoy a quality user experience.  Please select from one of the below API providers.\n\n");
        is_required = true;
        show_remote_python = true;
    } else {
        cli_header("Optional API Client");
        cli_send!("Although you've already defined a conversational model, you may optionally define an API integration which can be used as a fallback when Cicero needs outside help.  External APIs will only be contacted with your explicit permission.\n\n");
    }

    // Get model
    let (client_slug, api_key) = api_selector::select(is_required, show_remote_python);
    if router.default_model.is_empty() && !client_slug.is_empty() {
        router.default_model = client_slug.clone();
    }

    // Update config models, if remote api client
    if router.default_model == "remote_python_api".to_string() {
        config.models.summarization = "remote_python_api".to_string();
        config.models.text_generation = "remote_python_api".to_string();
        config.models.question_answer = "remote_python_api".to_string();
    }

    // Get Python API info, if needed
    if router.default_model == "remote_python_api".to_string() || client_slug == "remote_python".to_string() {
        cli_header("Remote Python API");
        cli_send!("Enter the host / IP address, port and API key of the remote Python API below.  Please refer to documentation if you need assistnace.\n\n");
        let api_host = cli_get_input("Host / IP Address: ", "");
        let api_port = cli_get_input("Port [7512: ", "7512");
        config.general.ollama_url = format!("http://{}:{}", api_host, api_port);
        let python_api_key = cli_get_input("API Key: ", "");
        *config.api_keys.entry("remote_python_api".to_string()).or_default() = api_key.to_string();
    }

    // Update config
    config.general.api_client = client_slug.clone();
    router.save();

    // Add api key to config
    if client_slug.starts_with("api:openai:") {
        *config.api_keys.entry("openai".to_string()).or_default() = api_key.to_string();
    } else if client_slug.starts_with("api:mistral:") {
        *config.api_keys.entry("mistral".to_string()).or_default() = api_key.to_string();
    } else if client_slug.starts_with("api:claude:") {
        *config.api_keys.entry("claude".to_string()).or_default() = api_key.to_string();
    }

}

/// Ask to help Ciero development
fn ask_help_cicero(config: &mut CiceroServerConfig) {

    cli_header("Help Improve Cicero");
    cli_send!("Thank you for installing Cicero. We're constantly working to enhance the software and provide you with the best possible AI assistant experience. Your anonymous data can play a crucial role in improving Cicero's natural language understanding (NLU) capabilities.\n\n"); 
    cli_send!("Rest assured, all shared data undergoes rigorous anonymization processes to protect your privacy. By contributing, you'll be making a significant impact on Cicero's development. You can review our data collection practices in detail by visiting our privacy policy at:\n\n");
    cli_send!("    https://cicero.sh/privacy.\n\n");

    config.general.agree_share = cli_confirm("Would you be willing to share some anonymized data to help improve Cicero?");
    println!("");
}

/// Create vault
fn create_vault(uuid: &Uuid) -> Vault {

    // Save .password file
    let password = random::generate_password(32);
    let password_file = format!("{}/manager/.password", sys::get_datadir());
    match fs::write(&password_file, &password) {
        Ok(_) => {  },
        Err(e) => {
            error!("Unable to write to file at {}, error: {}", password_file, e);
            std::process::exit(1);
        }
    };

    // Create and save vault
    let mut vault = Vault::create(&password, &uuid, None);
    vault.save();

    vault
}

/// Finish
fn success_message(config: &CiceroServerConfig, vault: &mut Vault) {

    // Start apollo server
    let mut proc_manager = ProcessManager::new();

    cli_header("Cicero Setup Complete!");
    cli_send!("Great, Cicero is all setup!  An encryption password has been randomly generated and placed within the file:\n    {}/manager/.password\n\n", sys::get_datadir());
    cli_send!("Nothing needs to be done, and you can now enjoy using Cicero.  However, if you prefer enhanced security you may optionally save the contents of the .password file and delete it, then you will be prompted for the password each time the server boots.\n\n");

        cli_send!("Is this machine intended to be a Cicero server only, or will you also be interacting with Cicero on this machine?\n\n");

    let mut options = IndexMap::new();
    options.insert("1".to_string(), "Continue and create profile on this machine.".to_string());
    options.insert("2".to_string(), "Complete setup, and connect to Cicero from another machine".to_string());

    let option = cli_get_option("How would you like to proceed?", &options);
    if option == "2".to_string() {
        proc_manager.start("apollo", 7511, 1);
        cli_send!("Sounds great, see you on the other side!  Upon running Cicero on other machines, simply enter the host {} and port 7511 when prompted and it will connect to this machine.\n\n", config.daemons.apollo.0);
        std::process::exit(0);
    }

    // Generate cfs directory
    let homedir = dirs::home_dir().expect("Unable to determine home directory");
    let mut cfs_dir = format!("{}{}cicero", homedir.to_str().unwrap().trim_end_matches(MAIN_SEPARATOR).to_string(), MAIN_SEPARATOR);

    if !Path::new(&cfs_dir.clone()).exists() {
        match fs::create_dir_all(&cfs_dir) {
            Ok(_) => { },
            Err(e) => {
                error!("Unable to create local directory at, {}", cfs_dir);
                cfs_dir = "".to_string();
            }
        };
    }

    // Create client config
    let mut client_config = CiceroClientConfig::default();
    client_config.is_first_time = true;
    client_config.cfs_dir = cfs_dir.clone();
    client_config.daemons = config.daemons.clone();
    client_config.apollo_api_key = vault.generate_apollo_api_key(None);
    client_config.save();

    // Save vault
    vault.save();

    // Start apollo server
    proc_manager.start("apollo", 7511, 1);
    println!("\n");
}

