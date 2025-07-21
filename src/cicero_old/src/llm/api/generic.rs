
use super::ApiAdapter;
use crate::error::Error;
use crate::server::CONFIG;

#[derive(Debug, Clone)]
pub struct Generic {
    pub chat_model: String,
    pub embed_model: String,
    pub chat_endpoint: String,
    pub embed_endpoint: String
}
impl Generic {
    pub fn new(parts: &[String]) -> Self {
        let mut adapter = Self::default();
        adapter.chat_endpoint = format!("http://{}/chat", parts.to_vec().join(":").to_string());
        adapter
    }
}



impl Default for Generic {
    fn default() -> Generic {

        Self {
            chat_model: "mistral-small".to_string(),
            embed_model: "mistral-embed".to_string(),
            chat_endpoint: "https://api.mistral.ai/v1/chat/completions".to_string(),
            embed_endpoint: "https://api.mistral.ai/v1/embeddings".to_string()
        }
    }
}

impl ApiAdapter for Generic {

    fn get_api_key(&self) -> Result<String, Error> {
        return Ok("abc1234".to_string());
        let api_key = match CONFIG.api_keys.get("generic_python") {
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


