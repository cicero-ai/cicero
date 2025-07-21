
use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;
use indexmap::IndexMap;
use atlas_http::{HttpClient, HttpBody, HttpRequest};
use regex::Regex;
use std::io;

// Represents a profile for interacting with an LLM provider's API
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct LlmProfile {
    pub provider: LlmProvider,    // The LLM provider (e.g., OpenAI, Ollama)
    pub api_key: String,          // API key for authentication
    pub model_name: String,       // Name of the specific model to use
    pub temperature: Option<f32>, // Optional temperature setting for generation
    pub max_tokens: Option<usize>, // Optional maximum token limit
    pub api_url: String,          // Custom API URL (if empty, uses provider default)
}

// Enum representing supported LLM providers
#[derive(Default, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum LlmProvider {
    #[default]
    ollama,    // Ollama (local LLM server)
    openai,    // OpenAI
    anthropic, // Anthropic
    google,    // Google Gemini
    xai,       // X.ai
    mistral,   // Mistral
    deepseek,  // Deepseek
    groq,      // Groq
    together,  // TogetherAI
    other      // Custom or unspecified provider
}

// Represents a single message in a chat conversation
#[derive(Serialize, Deserialize)]
struct ChatMessage {
    role: String,    // Role of the message sender (e.g., "user", "assistant")
    content: String, // Message content
}

// Standard chat request structure for most providers
#[derive(Serialize, Deserialize)]
struct ChatRequest {
    model: String,           // Model name to use
    messages: Vec<ChatMessage>, // List of messages in the conversation
    temperature: Option<f32>,   // Temperature for generation
    max_tokens: Option<usize>,  // Maximum tokens to generate
}

// Ollama-specific chat request structure
#[derive(Serialize, Deserialize)]
struct ChatRequest_Ollama {
    model: String,           // Model name to use
    messages: Vec<ChatMessage>, // List of messages in the conversation
    stream: bool,            // Whether to stream the response (false for single response)
    temperature: Option<f32>,   // Temperature for generation
    max_tokens: Option<usize>,  // Maximum tokens to generate
}

// Represents a single response choice from the LLM
#[derive(Serialize, Deserialize)]
struct ChatChoice {
    message: ChatMessage, // The generated message
}

// Standard chat response structure
#[derive(Serialize, Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>, // List of response choices (usually one for single requests)
}

impl LlmProfile {
    // Sends a single message to the LLM and returns the response
    pub fn send_single(&self, message: &str) -> Result<String, io::Error> {
        // Handle Ollama separately due to its unique API
        if self.provider == LlmProvider::ollama {
            return self.send_ollama(message);
        }

        // Construct a standard chat request
        let request = ChatRequest {
            model: self.model_name.clone(),
            messages: vec![ChatMessage {
                role: "user".to_string(),
                content: message.to_string(),
            }],
            temperature: self.temperature,
            max_tokens: self.max_tokens,
        };

        // Serialize the request to JSON
        let json_str = serde_json::to_string(&request)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
        self.send(&json_str)
    }

    // Sends a single message to Ollama's API and returns the response
    pub fn send_ollama(&self, message: &str) -> Result<String, io::Error> {
        // Construct an Ollama-specific chat request
        let request = ChatRequest_Ollama {
            model: self.model_name.clone(),
            stream: false, // Non-streaming response
            messages: vec![ChatMessage {
                role: "user".to_string(),
                content: message.to_string(),
            }],
            temperature: self.temperature,
            max_tokens: self.max_tokens,
        };

        // Serialize the request to JSON
        let json_str = serde_json::to_string(&request)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
        self.send(&json_str)
    }

    // Internal method to send an HTTP request to the LLM provider's API
    fn send(&self, json_str: &str) -> Result<String, io::Error> {
        // Prepare the request body
        let body = HttpBody::from_raw_str(json_str);
        let auth_header = format!("Authorization: Bearer {}", self.api_key);
        
        // Determine the API URL (custom or provider default)
        let mut url = if self.api_url.is_empty() {
            self.provider.get_completion_url()
        } else {
            self.api_url.clone()
        };

        // Substitute model name and API key in the URL if present
        url = url.replace("~model~", &self.model_name);
        url = url.replace("~api_key~", &self.api_key);

        // Build and send the HTTP POST request
        let req = HttpRequest::new("POST", &url, &vec![&auth_header.as_str()], &body);
        let mut http = HttpClient::builder().browser().build_sync();
        let res = http.send(&req)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        // Check for HTTP success status
        if res.status_code() != 200 {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("HTTP error: {}", res.status_code())
            ));
        }

        // Handle response based on provider
        if self.provider == LlmProvider::ollama {
            // Ollama returns a single ChatChoice directly
            let json_res: ChatChoice = serde_json::from_str(&res.body())
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
            Ok(json_res.message.content.clone())
        } else {
            // Other providers return a ChatResponse with choices
            let json_res: ChatResponse = serde_json::from_str(&res.body())
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
            if json_res.choices.is_empty() {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "No choices in response"
                ));
            }
            Ok(json_res.choices[0].message.content.clone())
        }
    }

    // Extracts JSON from a string, either an object or array
    pub fn extract_json<T: DeserializeOwned>(&self, input: &str, is_object: bool) -> Option<T> {
        let start_char = if is_object { '{' } else { '[' };
        let start_idx = input.find(start_char)?;
        
        // Track nesting depth to find complete JSON structure
        let mut depth = 0;
        let end_char = if is_object { '}' } else { ']' };
        let mut end_idx = 0;
        
        for (i, c) in input[start_idx..].char_indices() {
            if c == start_char {
                depth += 1;
            } else if c == end_char {
                depth -= 1;
                if depth == 0 {
                    end_idx = start_idx + i + 1;
                    break;
                }
            }
        }
        
        // Return None if no valid JSON structure found
        if end_idx == 0 || depth != 0 {
            return None;
        }
        
        let json_str = &input[start_idx..end_idx];
        
        // Attempt to deserialize the extracted JSON
        serde_json::from_str(json_str).ok()
    }

    // Flexibly extracts JSON (object, array, or scalar) from a string
    pub fn extract_json_flexible<T: DeserializeOwned>(&self, input: &str) -> Option<T> {
        // Try object first
        if let Some(result) = self.extract_json::<T>(input, true) {
            return Some(result);
        }
        
        // Then try array
        if let Some(result) = self.extract_json::<T>(input, false) {
            return Some(result);
        }
        
        // Try string value
        let re_string = Regex::new(r#""([^"\\]|\\[\s\S])*""#).ok()?;
        if let Some(m) = re_string.find(input) {
            if let Ok(value) = serde_json::from_str::<T>(m.as_str()) {
                return Some(value);
            }
        }
        
        // Try scalar value (number, boolean, null)
        let re_scalar = Regex::new(r"\b(true|false|null|-?\d+(\.\d+)?([eE][+-]?\d+)?)\b").ok()?;
        if let Some(m) = re_scalar.find(input) {
            if let Ok(value) = serde_json::from_str::<T>(m.as_str()) {
                return Some(value);
            }
        }
        
        None
    }

    // Creates an LlmProfile from string parameters
    pub fn from_str(
        provider_slug: &str,
        model_name: &str,
        api_key: &str,
        temperature: Option<f32>,
        max_tokens: Option<usize>,
    ) -> Self {
        LlmProfile {
            provider: LlmProvider::from_str(provider_slug),
            api_key: api_key.to_string(),
            model_name: model_name.to_string(),
            temperature,
            max_tokens,
            api_url: String::new(), // Default to empty; filled by provider if needed
        }
    }
}

impl LlmProvider {
    // Converts a numeric index to an LlmProvider
    pub fn from_usize(value: usize) -> Self {
        match value {
            0 => Self::ollama,
            1 => Self::openai,
            2 => Self::anthropic,
            3 => Self::google,
            4 => Self::xai,
            5 => Self::mistral,
            6 => Self::deepseek,
            7 => Self::groq,
            8 => Self::together,
            _ => Self::other
        }
    }

    // Returns a human-readable string representation of the provider
    pub fn to_string(&self) -> String {
        match self {
            Self::ollama => "Ollama".to_string(),
            Self::openai => "OpenAI".to_string(),
            Self::anthropic => "Anthropic".to_string(),
            Self::google => "Google Gemini".to_string(),
            Self::xai => "X.ai".to_string(),
            Self::mistral => "Mistral".to_string(),
            Self::deepseek => "Deepseek".to_string(),
            Self::groq => "Groq".to_string(),
            Self::together => "TogetherAI".to_string(),
            _ => "Other".to_string()
        }
    }

    // Returns a slug (lowercase identifier) for the provider
    pub fn to_slug(&self) -> String {
        match self {
            Self::ollama => "ollama".to_string(),
            Self::openai => "openai".to_string(),
            Self::anthropic => "anthropic".to_string(),
            Self::google => "google".to_string(),
            Self::xai => "xai".to_string(),
            Self::mistral => "mistral".to_string(),
            Self::deepseek => "deepseek".to_string(),
            Self::groq => "groq".to_string(),
            Self::together => "together".to_string(),
            _ => "other".to_string()
        }
    }

    // Returns a map of numeric indices to provider names
    pub fn get_indexmap_options() -> IndexMap<String, String> {
        let mut options = IndexMap::new();
        for x in 1..9 {
            let val = Self::from_usize(x);
            options.insert(format!("{}", x), val.to_string());
        }
        options
    }

    // Returns the default completion URL for the provider
    fn get_completion_url(&self) -> String {
        match self {
            LlmProvider::ollama => "http://localhost:11434/api/chat".to_string(),
            LlmProvider::openai => "https://api.openai.com/v1/chat/completions".to_string(),
            LlmProvider::anthropic => "https://api.anthropic.com/v1/messages".to_string(),
            LlmProvider::google => "https://generativelanguage.googleapis.com/v1beta/models/~model~:generateContent?key=~api_key~".to_string(),
            LlmProvider::xai => "https://api.x.ai/v1/chat/completions".to_string(),
            LlmProvider::mistral => "https://api.mixtral.ai/v1/chat/completions".to_string(),
            LlmProvider::deepseek => "https://api.deepseek.com/v1/chat/completions".to_string(),
            LlmProvider::groq => "https://api.groq.com/openai/v1/chat/completions".to_string(),
            LlmProvider::together => "https://api.together.xyz/v1/chat/completions".to_string(),
            LlmProvider::other => "http://localhost:8000/v1/chat/completions".to_string(),
        }
    }

    // Creates an LlmProvider from a slug string
    fn from_str(slug: &str) -> Self {
        match slug.to_lowercase().as_str() {
            "ollama" => LlmProvider::ollama,
            "openai" => LlmProvider::openai,
            "anthropic" => LlmProvider::anthropic,
            "google" => LlmProvider::google,
            "xai" => LlmProvider::xai,
            "mistral" => LlmProvider::mistral,
            "deepseek" => LlmProvider::deepseek,
            "groq" => LlmProvider::groq,
            "together" => LlmProvider::together,
            _ => LlmProvider::other,
        }
    }
}


