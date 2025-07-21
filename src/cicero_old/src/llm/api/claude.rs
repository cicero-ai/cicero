
use super::ApiAdapter;
use crate::error::Error;
use crate::server::CONFIG;

#[derive(Debug, Clone)]
pub struct Claude { 
    pub chat_model: String,
    pub embed_model: String,
    pub chat_endpoint: String,
    pub embed_endpoint: String
}

impl Claude {
    pub fn new(model: &str) -> Self {
        let mut adapter = Self::default();
        //adapter.chat_model = "".to_string9);
        adapter
    }
}


impl Default for Claude {
    fn default() -> Claude {

        Self {
            chat_model: "claude-3-sonnet-20240229".to_string(),
            embed_model: "claude-embed".to_string(),
            chat_endpoint: "https://api.anthropic.com/v1/messages".to_string(),
            embed_endpoint: "https://api.anthropic.com/v1/embeddings".to_string()
        }
    }
}

impl ApiAdapter for Claude {

    fn get_api_key(&self) -> Result<String, Error> {
        let api_key = match CONFIG.api_keys.get("claude") {
            Some(r) => r,
            None => { return Err( Error::NoConfig("Claude API key".to_string()) ); }
        };
        Ok(api_key.clone())
    }

    fn get_chat_model(&self) -> String {
        self.chat_model.clone()
    }

    fn get_embed_model(&self) -> String {
        self.embed_model.clone()
    }

    fn get_chat_url(&self) -> String {
        self.chat_endpoint.clone()
    }

    fn get_embed_url(&self) -> String {
        self.embed_endpoint.clone()
    }

    fn get_http_headers(&self) -> Vec<String> {
        let auth_line = format!("x-api-key: {}", self.get_api_key().unwrap());
        vec![
            auth_line,
            "anthropic-version: 2023-06-01".to_string()
        ]
    }

}


