
use serde_derive::{Serialize, Deserialize};
use super::ChatMessage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalChat {
    context: Vec<String>,
    messages: Vec<ChatMessage>
}

impl InternalChat {

    pub fn new() -> Self {
        Self {
            context: Vec::new(),
            messages: Vec::new()
        }
    }

    /// Ask for search query answer
    pub fn search_query(&mut self, query: &str, candidates: &Vec<String>) {
        self.add_system("Your assistance is needed with the NLU engine to understand the user's intent.  The below line prefixed with |CANDIDATES| is a comma delimited list of potential answers.  Use the user input below in this chat template, and reply in the specific format of \"[ANSWER]|[SIGNIFICANT_USER_INPUT]|[EXTRA]\", so a three part answer separated by the pipeline | character, where [ANSWER] is an exact match of one of the below potential answers, [SIGNIFICANT_USER_INPUT] is any optional segment of the user input that is vital in determining the answer, and [EXTRA] is any additional information you would like the NLU developers to have for future expansion of the NLU engine.");
        self.add_system( format!("|CANDIDATES| {}", candidates.join(", ").to_string() ).as_str());
        self.add_user(&query);
    }

    /// Add system message
    pub fn add_system(&mut self, message: &str) {
        self.context.push(message.to_string());
    }

    /// Add user
    pub fn add_user(&mut self, message: &str) {
        self.messages.push( ChatMessage { role: "user".to_string(), content: message.to_string() });
    }

    /// Add assistant
    pub fn add_assistant(&mut self, message: &str) {
        self.messages.push( ChatMessage { role: "assistant".to_string(), content: message.to_string() });
    }

    /// Format prompt
    fn format_prompt(&self) -> String {

        // Set messages
        let mut messages: Vec<ChatMessage> = vec![
            ChatMessage { role: "system".to_string(), content: "You are a helpful AI assistant named Cicero.  This is an internal, behind the scenes conversation with another machine as your assistance is needed.".to_string() }
        ];

        // Add context
        for message in self.context.iter() {
            messages.push( ChatMessage { role: "system".to_string(), content: message.to_string() });
        }

        // Add user / assistant messages
        for msg in self.messages.iter() {
            messages.push(msg.clone());
        }

        let json_str = serde_json::to_string(&messages).unwrap();
        json_str.to_string()
    }

}


