
use std::collections::LinkedList;
use serde_derive::{Serialize, Deserialize};
use atlas_http::{HttpClient, HttpRequest, HttpBody};
use crate::error::Error;
use super::ApiAdapter;

pub struct ApiClient {
    adapter: Box<dyn ApiAdapter>,
    chat: ApiChatRequest 
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiChatRequest {
    pub model: String,
    pub max_tokens: u16,
    pub temperature: f32,
    pub messages: LinkedList<ApiChatMessage>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiChatMessage {
    pub role: String,
    pub content: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiChatResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: LinkedList<ApiChatResponseChoices>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiChatResponseChoices {
    pub index: u16,
    pub message: ApiChatMessage
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponseUsage {
    pub prompt_tokens: u16,
    pub total_tokens: u16,
    pub completion_tokens: u16
}


impl ApiClient {

    pub fn new(adapter: Box<dyn ApiAdapter>) -> Self {

        let chat = ApiChatRequest {
            model: adapter.get_chat_model(),
            max_tokens: 3000,
            temperature: 0.5,
            messages: LinkedList::new()
        };

        let mut client = Self {
            adapter,
            chat
        };

        client.add_context("You are talking to a computer.  Try to keep all responses simple, machine readable without additional unnecessary text or descriptions.");
        client
    }

    /// Add context message to the chat
    pub fn add_context(&mut self, context: &str) {
        self.add_message("system", &context);
    }

    /// Add user prompt to the chat
    pub fn add_prompt(&mut self, prompt: &str) {
        self.add_message("user", &prompt);
    }

    /// Add message to the chat
    fn add_message(&mut self, role: &str, prompt: &str) {
        self.chat.messages.push_back( ApiChatMessage {
            role: role.to_string(),
            content: prompt.to_string()
        });
    }

    /// Send new chat request to Mistral
    pub fn send_chat(&mut self, prompt: &str) -> Result<String, Error> {

        // Set headers
        let mut headers = vec![
            "Content-Type: application/json",
            "Accept: application/json",
        ];

        // Add authorization line to headers
        let auth_headers = self.adapter.get_http_headers();
        for line in auth_headers.iter() {
            headers.push(&line.as_str());
        }

        // Get request body
        let json = self.create_chat_request(&prompt);
        let body = HttpBody::from_raw(&json.as_str().as_bytes());

        // Create http client
        let mut http = HttpClient::builder().build_sync();
        let req = HttpRequest::new("POST", &self.adapter.get_chat_url().as_str(), &headers, &body);

        // Send request
        let response = http.send(&req).unwrap();

        // Convert response to JSON
        let res: ApiChatResponse = match serde_json::from_str(&response.body()) {
            Ok(r) => r,
            Err(e) => { return Err( Error::ApiInvalidResponse(e.to_string()) ); }
        };

        // Get chat message
        let msg = match res.choices.iter().next() {
            Some(r) => r,
            None => { return Err( Error::ApiNoResults(true) ); }
        };

        // Add assistant message to chat
        let message = msg.message.content.clone();
        self.add_message("assistant", &message);

        Ok(message)
    }

    /// Create JSON chat request
    fn create_chat_request(&mut self, prompt: &str) -> String {

        // Add user message
        if !prompt.is_empty() {
            self.add_prompt(&prompt);
        }

        serde_json::to_string(&self.chat).unwrap()
    }

}


