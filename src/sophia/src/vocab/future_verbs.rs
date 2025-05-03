use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A trie-like structure for storing future verb phrases, tracking completion status and expected verb POS tags.
#[derive(Serialize, Deserialize)]
pub struct FutureVerbPhrases {
    pub is_complete: bool,
    pub expected_verb_pos: Option<String>,
    pub children: HashMap<String, Box<FutureVerbPhrases>>,
}

impl FutureVerbPhrases {
    /// Creates a new FutureVerbPhrases node with an optional expected verb POS tag and empty children.
    pub fn new(expected_verb_pos: Option<String>) -> Self {
        Self {
            is_complete: false,
            expected_verb_pos,
            children: HashMap::new(),
        }
    }

    /// Inserts a phrase into the trie, marking the final node as complete and handling verb placeholders.
    pub fn insert(&mut self, phrase: &str) {
        let mut current = self;
        for word in phrase.split(" ").collect::<Vec<&str>>().iter() {
            let child = if word.starts_with("V") {
                "[verb]".to_string()
            } else {
                word.to_string()
            };
            let expected_verb = if word.starts_with("V") {
                Some(word.to_string())
            } else {
                None
            };

            current = current
                .children
                .entry(child.to_lowercase())
                .or_insert(Box::new(FutureVerbPhrases::new(expected_verb)));
        }

        current.is_complete = true;
    }
}
