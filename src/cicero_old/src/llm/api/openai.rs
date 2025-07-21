
use super::ApiAdapter;
use crate::error::Error;
use crate::server::CONFIG;

#[derive(Debug, Clone)]
pub struct OpenAI {
pub     chat_model: String,
    pub embed_model: String,
    pub chat_endpoint: String,
    pub embed_endpoint: String
}

impl OpenAI {
    pub fn new(model: &str) -> Self {
        let mut adapter = Self::default();
        adapter.chat_model = model.to_string();
        adapter
    }
}


impl Default for OpenAI {
    fn default() -> OpenAI {

        Self {
            chat_model: "gpt-3.5-turbo-16k".to_string(),
            embed_model: "text-embedding-ada-002".to_string(),
            chat_endpoint: "https://api.openai.com/v1/chat/completions".to_string(),
            embed_endpoint: "https://api.openai.com/v1/embedings".to_string()
        }
    }
}

impl ApiAdapter for OpenAI {

    fn get_api_key(&self) -> Result<String, Error> {
        let api_key = match CONFIG.api_keys.get("openai") {
            Some(r) => r,
            None => { return Err( Error::NoConfig("OpenAI API key".to_string()) ); }
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
        let auth_line = format!("Authorization: Bearer {}", self.get_api_key().unwrap());
        vec![auth_line]
    }

}


