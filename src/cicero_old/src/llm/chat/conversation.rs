

use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::server::apollo::user::ServerUser;
use cicero_sdk::chat::{ChatKit, ChatNode};
use crate::llm::api::ApiClient;

#[derive(Serialize, Deserialize)]
pub struct Conversation {
    pub id: Uuid,
    pub uuid: Uuid,
    pub nickname: String,
    pub start_time: DateTime<Utc>,
    pub last_active: DateTime<Utc>,
    pub tagline: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub kit: Option<ChatKit>,
    pub context: Vec<String>,
    pub messages: Vec<ChatMessage>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String
}

impl Conversation {

    pub fn new(uuid: &Uuid, nickname: &str) -> Self {
        let mut conv = Self::default();
        conv.uuid = uuid.clone();
        conv.nickname = nickname.to_string();
        conv
    }

    /// Add message
    fn add(&mut self, role: &str, message: &str) {
        self.messages.push(ChatMessage {
            role: role.to_string(),
            content: message.to_string()
        });
    }

    /// Add system message
    pub fn add_system(&mut self, message: &str) {
        self.add("system", &message);
    }

    /// Add user message
    pub fn add_user(&mut self, auth_user: Option<Arc<Mutex<ServerUser>>>, message: &str) {




        // Add message
        self.add("user", &message);

    }

    /// Add assistant message
    pub fn add_assistant(&mut self, message: &str) {
        self.add("assistant", &message);
    }

    /// Reset context
    pub fn reset_context(&mut self) {
        self.context = Vec::new();
    }

    /// Format prompt for text generation pipeline
    pub fn get_messages_vec(&self) -> Vec<ChatMessage> {

        let mut messages: Vec<ChatMessage> = vec![
            ChatMessage { role: "system".to_string(), content: r#"You are a friendly, relaxed AI assistant named Cicero who has persistent memory and the ability to carry on conversations for weeks.  All system messages will be prefixed with one of the following tags:  |INSTRUCT\ Instructions you must follow. |RELAY| Relay to user in conversational format. |NAME| Name of the human you are assisting.. |MEM| From your persistent memory, use this as context to generate personalized response, |INFO| additional info regarding your capabilities that may be helpful with current conversation"#.to_string() }
        ];

        // Add context
        for msg in self.context.iter() {
            messages.push(ChatMessage { role: "system".to_string(), content: msg.to_string() });
        }

        // Add node, if we have one
        if self.kit.is_some() {
            for msg in self.kit.as_ref().unwrap().current_node.instruct.iter() {
                messages.push(ChatMessage { role: "system".to_string(), content: format!("|INSTRUCT| {}", msg) });
            }

            for msg in self.kit.as_ref().unwrap().current_node.relay.iter() {
                messages.push(ChatMessage { role: "system".to_string(), content: format!("|RELAY| {}", msg) });
            }
        }

        // Add nickname, if needed
        if !self.nickname.is_empty() {
            messages.push(ChatMessage { role: "system".to_string(), content: format!("|NAME| {}", self.nickname) });
        }

        // Add previous user / assistant messages
        if self.messages.len() > 5 {
            messages.extend(self.messages[..messages.len() - 5].to_vec());
        } else {
            messages.extend(self.messages.clone());
        }

        messages
    }

    // Formta prompt for LLM
    pub fn format_prompt(&self) -> String {
        let messages = self.get_messages_vec();
        let prompt = format!("Using the below chat template, respond with the next message.\n\n{}\n", serde_json::to_string(&messages).unwrap());
        prompt
    }
}

impl std::default::Default for Conversation {
    fn default() -> Conversation {
        Conversation {
            id: Uuid::new_v4(),
            uuid: Uuid::new_v4(),
            nickname: String::new(),
            start_time: Utc::now(),
            last_active: Utc::now(),
            tagline: String::new(),
            kit: None,
            context: Vec::new(),
            messages: Vec::new()
        }
    }
}


