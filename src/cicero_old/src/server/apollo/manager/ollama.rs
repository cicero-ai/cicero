
use falcon_cli::*;
use std::collections::HashMap;
use crate::llm::models::ModelSize;
use crate::server::config::CiceroServerConfig;
use crate::llm::chat::ChatRouter;
use super::hq::ConversationModel;

/// Setup Ollama 
pub fn setup(size: &ModelSize, models: &Vec<ConversationModel>, config: &mut CiceroServerConfig) -> ChatRouter {

    // Check if Ollama installed locally
    if let Ok(res) = which::which("ollama") {
        config.general.ollama_url = choose_installation();
    } else {
        prompt_install();
    }

    let mut router = ChatRouter::default();
    if !config.general.ollama_url.is_empty() {
        let model = select_model(&size, &models);
        router.default_model = format!("ollama:{}", model); 
    }
    router
}

/// Choose installation
fn choose_installation() -> String {

    cli_send!("Great News! Ollama is already installed on this machine. Please select one of the options below:\n\n");

    // Get option
    let mut options = IndexMap::new();
    options.insert("1".to_string(), "Use local Ollama installation (127.0.0.1:11434)".to_string());
    options.insert("2".to_string(), "Use Ollama, but specify a different IP address and port.".to_string());
    options.insert("3".to_string(), "Do not use Ollama, use an external API instead (OPENAI, Mistral, Claude, etc.)".to_string());
    let opt = cli_get_option("Select One: ", &options);

    if opt == "1".to_string() {
        return "127.0.0.1|11434".to_string();
    } else if opt == "3".to_string() {
        return String::new();
    }

    // Get host / port of remote Ollama installatoin
    println!("");
    cli_send!("Enter the host / IP address and portof a remote Ollama installation to use:\n\n");
    let host = cli_get_input("Host / IP Address [127.0.0.1]: ", "127.0.0.1");
    let port = cli_get_input("Port [11434]: ", "11434");

    format!("{}|{}", host, port)
}

/// Prompt user to install Ollama
fn prompt_install() {

}

/// Select the conversational model
fn select_model(size: &ModelSize, models: &Vec<ConversationModel>) -> String {

    // GEt size
    let mut size_str = serde_json::to_string(&size).unwrap().to_lowercase().trim_end_matches('"').trim_start_matches('"').to_string();
    if size_str == "tiny".to_string() { size_str = "small".to_string(); }
    if size_str == "extralarge".to_string() { size_str = "large".to_string(); }

    // Header
    cli_header("Select Conversation Model");
    cli_send!("Choose your desired conversation model from below, or simply press enter to accept the recommended default.  This model is only used to generate conversational replies, and will not affect the understanding or functionality of Cicero.\n\n");

    // Set varibles
    let mut size = String::new();
    let mut default = String::new();
    let mut options: HashMap<String, String> = HashMap::new();
    let mut x = 1;

    // Go through models
    for model in models {

        if size != model.size {
            cli_send!("\n    {}\n\n", model.size.to_uppercase().to_string());
            size = model.size.to_string();
        }

        // Check default
        let mut recommended = "";
        if model.is_default == 1 && size == size_str {
            default = model.alias.to_string();
            recommended = " (recommended)";
        }
        cli_send!("[{}] {}{}\n", x, model.name, recommended);

        // Add to options
        options.insert(format!("{}", x), model.alias.clone());
        x += 1;
    }
    cli_send!("\n[0] None of the above, specify model name.\n\n");

    // Select model
    let mut model_name = String::new();
    loop {
        let msg = format!("Select Model [{}]: ", default);
        let opt = cli_get_input(&msg.as_str(), &default.as_str());

        if opt == default {
            model_name = default.clone();
            break;
        } else if options.contains_key(&opt.to_string()) {
            model_name = options.get(&opt.to_string()).unwrap().to_string();
            break;
        } else if opt == "0".to_string() {
            cli_send!("\nEnter a valid Ollama model name below.\n\n");
            model_name = cli_get_input("Model Name: ", "");
            break;
        } else {
            cli_send!("Invalid option, please specify a valid model.\n\n");
        }
    }

    model_name
}


