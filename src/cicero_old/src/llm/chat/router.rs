
use std::collections::{HashMap, HashSet};
use serde_derive::{Serialize, Deserialize};
use crate::error::Error;
use crate::utils::sys;
use tokio::runtime::Runtime;
use tokio::net::TcpStream;
use ollama_rs::Ollama;
use ollama_rs::generation::completion::request::GenerationRequest;
use tokio::io::{stdout, AsyncWriteExt};
use tokio_stream::StreamExt;
use std::path::Path;
use std::fs;
use std::io::Write;
use crate::server::CONFIG;
use super::Conversation;

#[derive(Serialize, Deserialize)]
pub struct OllamaWord {
    pub response: String,
    pub done: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRouter {
    pub default_model: String,
    pub routes: HashMap<String, String>
}

impl ChatRouter {

    pub fn new() -> Self {

        let router_file = format!("{}/config/router.yml", sys::get_datadir());
        if !Path::new(&router_file).exists() {
            return Self::default();
        }

        let yaml_str = fs::read_to_string(&router_file).unwrap();
        let router: ChatRouter = serde_yaml::from_str(&yaml_str).unwrap();

        router
    } 

    /// Save router config
    pub fn save(&self) {
        let router_file = format!("{}/config/router.yml", sys::get_datadir());
        let yaml_str = serde_yaml::to_string(&self).unwrap();
        fs::write(&router_file, &yaml_str).expect("Unable to write to router.yml file");
    }

    /// Route message, and provide response from LLM
    pub async fn route_streamed(&mut self, conv: &Conversation, stream: &mut std::net::TcpStream) -> String {

        // Initialize
        let mut response = String::new();
        let prompt = conv.format_prompt();

        // Route to ollama
        if self.default_model.starts_with("ollama:") {
            response = self.send_ollama_streamed(&prompt, stream).await.unwrap();
        }

        response
    }

    /// Send to Ollama
    async fn send_ollama_streamed(&self, prompt: &String, stream: &mut std::net::TcpStream) -> Result<String, Error> {

        // Set variables
        let model_name = self.default_model[7..].to_string();
        let mut response = String::new();

        // Start ollama
        let ollama = Ollama::default();
        let mut res_stream = ollama.generate_stream(GenerationRequest::new(model_name.to_string(), prompt.to_string())).await.unwrap();

        // Send success http status header
        stream.write("HTTP/1.1 200 OK\n\n".as_bytes()).unwrap();
        stream.flush().unwrap();

        // Send request to ollama
        let mut stdout = tokio::io::stdout();
        while let Some(res) = res_stream.next().await {
            let responses = res.unwrap();
            for resp in responses {
                response.push_str(&resp.response.as_str());
                let word = OllamaWord { response: resp.response.clone(), done: resp.done.clone() };
                let json = format!("{}\n", serde_json::to_string(&word).unwrap());

                stream.write(json.as_bytes()).unwrap();
                stream.flush().unwrap();

                stdout.write(resp.response.as_bytes()).await.unwrap();
                stdout.flush().await.unwrap();
            }
        }

        Ok(response)
    }

}

impl Default for ChatRouter {
    fn default() -> ChatRouter {
        ChatRouter {
            default_model: String::new(),
            routes: HashMap::new()
        }
    }
}

