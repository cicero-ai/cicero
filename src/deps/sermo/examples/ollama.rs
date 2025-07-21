
use sermo::{LlmProfile, LlmProvider};

fn main() -> Result<(), std::io::Error> {
    // Create an LlmProfile for Ollama running locally
    let profile = LlmProfile {
        provider: LlmProvider::ollama,
        api_key: String::new(),
        model_name: "gemma3".to_string(),
        temperature: Some(0.7), // Moderate creativity
        max_tokens: Some(100),  // Limit response length
        api_url: String::new()
    };

    // Send a simple message and get the response
    let message = "Hello! Tell me something interesting about Rust.";
    let response = profile.send_single(message)?;

    // Print the result
    println!("Ollama response: {}", response);

    Ok(())
}


