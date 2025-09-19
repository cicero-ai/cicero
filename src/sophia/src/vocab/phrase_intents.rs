// Copyright 2025 Aquila Labs of Alberta, Canada <matt@cicero.sh>
// Licensed under the PolyForm Noncommercial License 1.0.0
// Commercial use requires a separate license: https://cicero.sh/sophia/
// License text: https://polyformproject.org/licenses/noncommercial/1.0.0/
// Distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND.

use crate::error::Error;
use crate::tokenizer::{Token, TokenizedInput};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// A trie-like structure for storing phrase intents
#[derive(Serialize, Deserialize)]
pub struct PhraseIntents {
    pub intent: Option<PhraseIntent>,
    pub children: HashMap<i32, Box<PhraseIntents>>,
}

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum PhraseIntent {
    acknowledgment,
    affirmation,
    emphasis,
    hesitation,
    negation,
    #[default]
    neutral,
    rejection,
    request,
}

impl Default for PhraseIntents {
    fn default() -> Self {
        Self::new()
    }
}

impl PhraseIntents {
    /// Creates a new phrase intents node
    pub fn new() -> Self {
        Self {
            intent: None,
            children: HashMap::new(),
        }
    }

    /// Inserts a phrase into the trie
    pub fn insert(&mut self, intent: PhraseIntent, tokens: &[Token]) {
        let mut current = self;
        for token in tokens.iter() {
            current = current.children.entry(token.index).or_insert(Box::new(PhraseIntents::new()));
        }
        current.intent = Some(intent);
    }

    /// Check a vector of tokens as to whether or not a phrase intent entry exists
    pub fn check(
        &self,
        mut position: usize,
        output: &TokenizedInput,
    ) -> Option<(PhraseIntent, usize)> {
        let mut length = 0;
        let mut index = if let Some(token) = &output.mwe[position].token {
            token.index
        } else {
            output.tokens[output.mwe[position].position].index
        };
        let mut current = self;
        while let Some(node) = current.children.get(&index) {
            length += 1;
            if let Some(intent) = node.intent {
                return Some((intent, length));
            }

            position += 1;
            if position >= output.mwe.len() {
                return None;
            }
            index = if let Some(child_token) = &output.mwe[position].token {
                child_token.index
            } else {
                output.tokens[output.mwe[position].position].index
            };
            current = node;
        }

        None
    }
}

impl PhraseIntent {
    pub fn from_str(value: &str) -> Result<Self, Error> {
        match value {
            "acknowledgment" => Ok(PhraseIntent::acknowledgment),
            "affirmation" => Ok(PhraseIntent::affirmation),
            "emphasis" => Ok(PhraseIntent::emphasis),
            "hesitation" => Ok(PhraseIntent::hesitation),
            "negation" => Ok(PhraseIntent::negation),
            "neutral" => Ok(PhraseIntent::neutral),
            "rejection" => Ok(PhraseIntent::rejection),
            "request" => Ok(PhraseIntent::request),
            _ => Err(Error::Generic(format!(
                "Invalid phrase intent value, {}",
                value
            ))),
        }
    }
}

impl fmt::Display for PhraseIntent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            PhraseIntent::acknowledgment => "acknowledgment".to_string(),
            PhraseIntent::affirmation => "affirmation".to_string(),
            PhraseIntent::emphasis => "emphasis".to_string(),
            PhraseIntent::hesitation => "hesitation".to_string(),
            PhraseIntent::negation => "negation".to_string(),
            PhraseIntent::neutral => "neutral".to_string(),
            PhraseIntent::rejection => "rejection".to_string(),
            PhraseIntent::request => "request".to_string(),
        };
        write!(f, "{}", value)
    }
}
