

# Sermo

A Rust client library for interacting with various Large Language Model (LLM) provider APIs.

[![Crates.io](https://img.shields.io/crates/v/sermo.svg)](https://crates.io/crates/sermo)
[![Docs.rs](https://docs.rs/sermo/badge.svg)](https://docs.rs/sermo)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- Supports multiple LLM providers (Ollama, OpenAI, Anthropic, Google, X.ai, Mistral, Deepseek, Groq, TogetherAI, and more)
- Simple API for sending chat messages and receiving responses
- Configurable model settings (temperature, max tokens)
- Flexible JSON extraction from responses

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
sermo = "0.1.0"

Usage
Here's a quick example of using Sermo with Ollama:
rust

use sermo::{LlmProfile, LlmProvider};

fn main() -> Result<(), std::io::Error> {
    let profile = LlmProfile {
        provider: LlmProvider::ollama,
        api_key: String::new(),
        model_name: "llama2".to_string(),
        temperature: Some(0.7),
        max_tokens: Some(100),
        api_url: "http://localhost:11434/api/chat".to_string(),
    };

    let response = profile.send_single("Hello! Tell me something about Rust.")?;
    println!("Response: {}", response);

    Ok(())
}

Run the example with:
bash

cargo run --example ollama

Supported Providers

    Ollama
    OpenAI
    Anthropic
    Google Gemini
    X.ai
    Mistral
    Deepseek
    Groq
    TogetherAI
    Custom (via other provider)

Documentation
Full documentation is available on Docs.rs.
Contributing
Contributions are welcome! Please open an issue or submit a pull request on GitHub.
License
This project is licensed under the MIT License - see the LICENSE file for details.
Author
Matt Dizak matt@cicero.sh (mailto:matt@cicero.sh)


