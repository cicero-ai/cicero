
use falcon_cli::*;
use falcon_cli::{indexmap, IndexMap};

/// Select an API client
pub fn select(is_required: bool, show_remote_python: bool) -> (String, String) {

    let mut providers = IndexMap::new();
    providers.insert("1".to_string(), "Mistral".to_string());
    providers.insert("2".to_string(), "Claude".to_string());
    providers.insert("3".to_string(), "OpenAI".to_string());
    if show_remote_python {
        providers.insert("4".to_string(), "Remote Python API (see documentation)".to_string());
    }
    if !is_required {
        providers.insert("5".to_string(), "None, continue without API.".to_string());
    }
    let provider = cli_get_option("Select your desired API provider: ", &providers);

    // Get api client
    let client_slug: String = match provider.as_str() {
        "1" => mistral(),
        "2" => claude(),
        "3" => openai(),
        "4" => "remote_python".to_string(),
        _ => String::new()
    };

    if client_slug.is_empty() || client_slug == "remote_python".to_string() {
        return (client_slug.clone(), String::new());
    }

    println!("");
    let api_key = cli_get_input("API Key: ", "");

    (client_slug, api_key)
}

/// Mistral
fn mistral() -> String {

    // Get size
    let mut options = IndexMap::new();
    options.insert("1".to_string(), "Large".to_string());
    options.insert("2".to_string(), "Medium".to_string());
    options.insert("3".to_string(), "Small".to_string());
    options.insert("4".to_string(), "Tiny".to_string());

    let selected = cli_get_option("Select a size: ", &options);
    let size = match selected.as_str() {
        "1" => "large",
        "2" => "medium",
        "3" => "small",
        "4" => "tiny",
        _ => panic!("Invalid option")
    };

    format!("api:mistral:{}", size)
}


fn claude() -> String {

    let mut options = IndexMap::new();
    options.insert("1".to_string(), "Opus".to_string());
    options.insert("2".to_string(), "Sonnet".to_string());
    options.insert("3".to_string(), "Haiku".to_string());

    let selected = cli_get_option("Select a size: ", &options);
    let size = match selected.as_str() {
        "1" => "claude-3-opus-20240229",
        "2" => "claude-3-sonnet-20240229",
        "3" => "claude-3-haiku-20240307",
        _ => panic!("Invalid option")
    };

    format!("api:claude:{}", size)
}
fn openai() -> String {
    "api:openai:gpt-4-turbo".to_string()
}









