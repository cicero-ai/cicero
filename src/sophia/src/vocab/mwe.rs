use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a multi-word entity (MWE) node in a trie-like structure, with index, type, capitalization, and child nodes.
#[derive(Default, Serialize, Deserialize)]
pub struct VocabMWE {
    pub index: i32,
    pub mwe_type: MWEType,
    pub capitalization: Capitalization,
    pub orig_word: String,
    pub children: HashMap<String, Box<VocabMWE>>,
}

/// Defines the type of a multi-word entity, which can be standard, scoring, or both.
#[derive(Default, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub enum MWEType {
    #[default]
    standard,
    scoring,
    both,
}

/// Defines the capitalization style of a word, which can be lowercase, uppercase, title case, or other.
#[derive(Default, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub enum Capitalization {
    #[default]
    lower,
    upper,
    title,
    other(String),
}

impl VocabMWE {
    /// Creates a new VocabMWE node for a word with the specified MWE type and inferred capitalization.
    pub fn new(word: &str, mwe_type: MWEType) -> VocabMWE {
        let capitalization = Self::classify_capitalization(word);

        VocabMWE {
            index: 0,
            mwe_type,
            capitalization,
            orig_word: String::new(),
            children: HashMap::new(),
        }
    }

    /// Inserts a phrase into the MWE trie, assigning the given index and MWE type, and returns the index.
    pub fn insert(&mut self, phrase: &str, index: i32, mwe_type: MWEType) -> i32 {
        let mut current = self;
        for word in phrase.split(" ").collect::<Vec<&str>>().iter() {
            current = current
                .children
                .entry(word.to_lowercase().to_string())
                .or_insert(Box::new(VocabMWE::new(word, mwe_type.clone())));

            if current.mwe_type == MWEType::standard && mwe_type == MWEType::scoring {
                current.mwe_type = MWEType::both;
            } else if current.mwe_type == MWEType::scoring && mwe_type == MWEType::standard {
                current.mwe_type = MWEType::both;
            }
        }

        current.index = index;
        index
    }

    /// Retrieves the index of a multi-word entity phrase from the trie, returning 0 if not found.
    pub fn get(&self, phrase: &str) -> i32 {
        let mut current = self;
        for word in phrase.to_lowercase().split(" ").collect::<Vec<&str>>().iter() {
            match current.children.get(&word.to_string()) {
                Some(next) => current = next.as_ref(),
                None => return 0,
            }
        }
        current.index
    }

    /// Classifies the capitalization style of a string (lowercase, uppercase, title case, or other).
    pub fn classify_capitalization(s: &str) -> Capitalization {
        if s.to_lowercase() == s {
            Capitalization::lower
        } else if s.to_uppercase() == s {
            Capitalization::upper
        } else if s.chars().all(|c| c.is_uppercase()) && s.chars().any(|c| c.is_lowercase()) {
            Capitalization::title
        } else {
            Capitalization::other(s.to_string())
        }
    }

    /// Formats a word according to the node's capitalization style (lowercase, uppercase, title case, or original).
    pub fn format(&self, word: &String) -> String {
        match self.capitalization {
            Capitalization::lower => word.to_lowercase(),
            Capitalization::upper => word.to_uppercase(),
            Capitalization::title => format!(
                "{}{}",
                word.chars().next().unwrap().to_uppercase(),
                &word[1..].to_lowercase()
            ),
            _ => self.orig_word.to_string(),
        }
    }
}

impl Capitalization {
    /// Creates a Capitalization variant from a string value, using the original string for 'other' cases.
    pub fn from_str(value: &str, orig: &str) -> Self {
        match value {
            "lower" => Self::lower,
            "upper" => Self::upper,
            "title" => Self::title,
            "other" => Self::other(orig.to_string()),
            _ => panic!("Invalid capitalization value, {}", value),
        }
    }
}
