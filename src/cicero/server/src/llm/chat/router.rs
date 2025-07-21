
use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};
use cicero::utils::sys;
use std::path::Path;
use std::fs;
use std::io::Write;
use crate::llm::models::{LlmProfile, LlmProvider};
use super::Conversation;
use crate::{Error, CONFIG};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct ChatRouter {
    pub default_llm: LlmProfile,
    pub llm_profiles: HashMap<String, LlmProfile>
}

impl ChatRouter {
    pub fn new() -> Self {

        let router_file = format!("{}/server/chat_router.json", sys::get_datadir());
        if !Path::new(&router_file).exists() {
            return Self::default();
        }

        let json_str = fs::read_to_string(&router_file).expect("Unable to read from chat_router.json file");
        let router: ChatRouter = serde_json::from_str(&json_str).expect("Unable to deserialize chat_router.json configuration file");

        router
    }

    /// Save router config
    pub fn save(&self) {
        let router_file = format!("{}/server/chat_router.json", sys::get_datadir());
        let json_str: String = serde_json::to_string(&self).unwrap();
        fs::write(&router_file, &json_str).expect("Unable to write to chat_router.json file");
    }

    /// Route message, and provide response from LLM
    pub async fn route_streamed(&mut self, conv: &Conversation, stream: &mut std::net::TcpStream) -> String {

        // Format the prompt
        let prompt = conv.format_prompt();
        let mut response = String::new();

        // Route to ollama
        if self.default_llm.provider == LlmProvider::ollama {
            //response = self.send_ollama_streamed(&prompt, stream).await.unwrap();
        }

        response
    }

}


