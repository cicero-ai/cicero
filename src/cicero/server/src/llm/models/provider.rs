
use serde::{Serialize, Deserialize};
use indexmap::IndexMap;
use atlas_http::{HttpClient, HttpBody, HttpRequest, HttpResponse};
use crate::Error;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct LlmProfile {
    pub provider: LlmProvider,
    pub api_key: String,
    pub model_name: String,
    pub temperature: Option<f32>,
    pub max_tokens: Option<usize>,
    pub api_url: String,
}

#[derive(Default, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum LlmProvider {
    #[default]
    ollama,
    openai,
    anthropic,
    google,
    xai,
    mistral,
    deepseek,
    groq,
    together,
    other
}

#[derive(Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: Option<f32>,
    max_tokens: Option<usize>,
}

#[derive(Serialize, Deserialize)]
struct ChatChoice {
    message: ChatMessage,
}

#[derive(Serialize, Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

impl LlmProfile {
    pub fn send_single(&self, message: &str) -> Result<String, Error> {
        let request = ChatRequest {
            model: self.model_name.clone(),
            messages: vec![ChatMessage {
                role: "user".to_string(),
                content: message.to_string(),
            }],
            temperature: self.temperature,
            max_tokens: self.max_tokens,
        };

        let json_str = serde_json::to_string(&request).map_err(|e| Error::Json(e.to_string()))?;
        self.send(&json_str)
    }

    fn send(&self, json_str: &str) -> Result<String, Error> {
        // Same HttpBody, HttpRequest, HttpSyncClient stubs as before...

        let mut body = HttpBody::from_raw_str(json_str);
        let auth_header = format!("Authorization: Bearer {}", self.api_key);
        let mut url = if self.api_url.is_empty() {
            self.provider.get_completion_url()
        } else {
            self.api_url.clone()
        };

        // Replace params in url, if needed
        url = url.replace("~model~", &self.model_name);
        url = url.replace("~api_key~", &self.api_key);

        let req = HttpRequest::new("POST", &url, &vec![&auth_header.as_str()], &body);
        let mut http = HttpClient::builder().browser().build_sync();
        let res = http.send(&req).map_err(|e| Error::Http(e.to_string()))?;

        if res.status_code() != 200 {
            return Err(Error::Generic(format!("HTTP error: {}", res.status_code())));
        }

        let json_res: ChatResponse = serde_json::from_str(&res.body()).map_err(|e| Error::Json(e.to_string()))?;
        if json_res.choices.is_empty() {
            return Err(Error::Generic("No choices in response".to_string()));
        }

        Ok(json_res.choices[0].message.content.clone())
    }

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
            api_url: String::new(), // Default to empty; provider URL filled later if needed
        }
    }
}



impl LlmProvider {
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

    pub fn get_indexmap_options() -> IndexMap<String, String> {

        let mut options = IndexMap::new();
        for x in 1..9 {
            let val = Self::from_usize(x);
            options.insert(format!("{}", x), val.to_string());
        }

        options
    }

    fn get_completion_url(&self) -> String {
        match self {
            LlmProvider::ollama => "http://localhost:11434/api/chat".to_string(),
            LlmProvider::openai => "https://api.openai.com/v1/chat/completions".to_string(),
            LlmProvider::anthropic => "https://api.anthropic.com/v1/messages".to_string(),
            LlmProvider::google => "https://generativelanguage.googleapis.com/v1beta/models/~model~:generateContent?key=~api_key~".to_string(),
            LlmProvider::xai => "https://api.x.ai/v1/chat/completions".to_string(),
            LlmProvider::mistral => "https://api.mixtral.ai/v1/chat/completions".to_string(), // Hypothetical cloud endpoint
            LlmProvider::deepseek => "https://api.deepseek.com/v1/chat/completions".to_string(),
            LlmProvider::groq => "https://api.groq.com/openai/v1/chat/completions".to_string(),
            LlmProvider::together => "https://api.together.xyz/v1/chat/completions".to_string(),
            LlmProvider::other => "http://localhost:8000/v1/chat/completions".to_string(), // Default for custom setups
        }
    }

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
            _ => LlmProvider::other, // Default for unrecognized slugs
        }
    }

}


