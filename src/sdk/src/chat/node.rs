
use serde_derive::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatNode {
    pub category: ChatNodeCategory,
    pub relay: Vec<String>,
    pub instruct: Vec<String>,
    do_sentiment_analysis: bool,
    do_ner: bool,
    //triggers: Vec<ChatPipelineTrigger>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChatNodeCategory {
    Neutral,
    //CollectDetails(Vec<RequestedItem>),
    AskQuestion,
    FacilitateExpansiveInput,
    Confirm,
    PresentChoices(HashMap<String, String>),
    GiveInstructions,
    Statement,
    ClarificationRequest,
    Apology,
    FeedbackRequest,
    SwitchContext,
    ProactiveNotification,
}

impl ChatNode {

    pub fn new() -> Self {
        Self::default()
    }

    /// Set the category
    pub fn set_category(&mut self, category: ChatNodeCategory) {
        self.category = category;
    }

    /// Add relay
    pub fn relay(&mut self, message: &str) {
        self.relay.push(message.to_string());
    }

    /// Add instructions
    pub fn instruct(&mut self, message: &str) {
        self.instruct.push(message.to_string());
    }

    /// Enable / disable sentiment analysis on next user input
    pub fn do_sentiment_analysis(&mut self, enable: bool) {
        self.do_sentiment_analysis = enable;
    }

    /// Enable ner (named entity recognition) on next user input
    pub fn do_ner(&mut self, enable: bool) {
        self.do_ner = enable;
    }

}

impl Default for ChatNode {
    fn default() -> ChatNode {
        ChatNode {
            category: ChatNodeCategory::Neutral,
            relay: Vec::new(),
            instruct: Vec::new(),
            do_sentiment_analysis: false,
            do_ner: false,
            //triggers: Vec::new()
        }
    }
}


