
use std::collections::HashMap;
use indexmap::IndexMap;
use falcon_cli::*;
use uuid::Uuid;
use crate::llm::models::{LlmProfile, LlmProvider, ModelSize};
use crate::llm::chat::ChatRouter;
use crate::apollo::hq::{CiceroHQ, ConversationModel};
use log::error;
use super::{ollama, HardwareProfile};

/// Get LLM provider for conversational output
pub fn get_provider(uuid: &Uuid) {

    // Get hardware profile
    let hw_profile = HardwareProfile::new();

    // Get default models
    let hq = CiceroHQ::new();
    let conversation_models = hq.get_default_models(&hw_profile.llm_size, &uuid);

    // Check for Ollama if hardware allows
    let mut is_ollama = false;
    if hw_profile.llm_size != ModelSize::tiny {
        match ollama::run() {
            Ok(status) => is_ollama = status,
            Err(e) => error!("Error installing Ollama, skipping.  error: {}", e)
        };
    }

    // Get default llm profile
    let default_llm: LlmProfile = if is_ollama {
        get_ollama_provider(&hw_profile.llm_size, &conversation_models)
    } else {
        get_external_provider(&conversation_models)
    };
    println!("");

    // Save chat router
    let router = ChatRouter {
        default_llm,
        ..Default::default()
    };
    router.save();

}

/// Get Olllama model name 
fn get_ollama_provider(size: &ModelSize, models: &Vec<ConversationModel>) -> LlmProfile {

    // Header
    let mut options: IndexMap<String, String> = IndexMap::new();
    let mut id2model: HashMap<String, String> = HashMap::new();

    // Go through models
    let mut x = 1;
    for model in models.iter() {
        if model.provider != "ollama".to_string() || model.size != size.to_string() { continue; }
        options.insert(format!("{}", x), model.name.to_string());
        id2model.insert(format!("{}", x), model.alias.to_string());
        x += 1;
    }

    // Add remaining options
    let (manual_x, skip_x) = (format!("{}", x), format!("{}", x+1));
    options.insert(format!("{}", manual_x.to_string()), "None, manually enter model name".to_string());
    options.insert(skip_x.to_string(), "Skip, use API provider (OpenAI, Anthropic, etc.)".to_string());

    // GEt option
    cli_send!("Select one of the Ollama models from below to use for general conversation:\n\n");
    let model_id = cli_get_option("Select Model: ", &options);

    // Get model name
    let mut model_name = String::new();
    if model_id == manual_x {
        model_name = cli_get_input("Model Name: ", "");
    } else if model_id == skip_x {
        return get_external_provider(&models);
    } else {
        model_name = id2model.get(&model_id).unwrap().to_string();
    }

    LlmProfile {
        provider: LlmProvider::ollama,
        model_name,
        ..Default::default()
    }
}

/// Get external API provider
fn get_external_provider(models: &Vec<ConversationModel>) -> LlmProfile {

    // Get options
    let options = LlmProvider::get_indexmap_options();

        // Get llm provider
    cli_send!("Select one of the API providers from below to use for general conversation:\n\n");
    let provider_id = cli_get_option("Select Provider: ", &options).parse::<usize>().unwrap();
    let provider = LlmProvider::from_usize(provider_id);
    let slug = provider.to_slug();

    // Get API key
    println!("");
        let api_key = cli_get_input(format!("{} API Key: ", provider.to_string()).as_str(), "");

    // Set variables
    let mut model_options = IndexMap::new();
    let mut id2model: HashMap<String, String> = HashMap::new();
    let mut x = 1;

    // Go through model names
    for model in models.iter() {
        if model.provider != slug { continue; }
        model_options.insert(format!("{}", x), model.name.to_string());
        id2model.insert(format!("{}", x), model.alias.to_string());
        x += 1;
    }

        // Select model name
    println!("");
    cli_send!("Select which model from {} you would like to use:\n\n", provider.to_string());
    let model_id = cli_get_option("Select One: ", &model_options);

    // Return
    LlmProfile {
        provider,
        api_key,
        model_name: id2model.get(&model_id).unwrap().to_string(),
        ..Default::default()
    }
}


