
use super::ApiAdapter;
use crate::error::Error;
use crate::server::CONFIG;

#[derive(Debug, Clone)]
pub struct Mistral { 
    pub chat_model: String,
    pub embed_model: String,
    pub chat_endpoint: String,
    pub embed_endpoint: String
}

impl Mistral {
    pub fn new(model: &str) -> Self {
        let mut adapter = Self::default();

        adapter
    }
}
impl Default for Mistral {
    fn default() -> Mistral {

        Self {
            chat_model: "mistral-small".to_string(),
            embed_model: "mistral-embed".to_string(),
            chat_endpoint: "https://api.mistral.ai/v1/chat/completions".to_string(),
            embed_endpoint: "https://api.mistral.ai/v1/embeddings".to_string()
        }
    }
}

impl ApiAdapter for Mistral {

    fn get_api_key(&self) -> Result<String, Error> {
        let api_key = match CONFIG.api_keys.get("mistral") {
            Some(r) => r,
            None => { return Err( Error::NoConfig("Mixtral API key".to_string()) ); }
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


