// Copyright 2025 Aquila Labs of Alberta, Canada <matt@cicero.sh>
// Licensed under the PolyForm Noncommercial License 1.0.0
// Commercial use requires a separate license: https://cicero.sh/sophia/
// License text: https://polyformproject.org/licenses/noncommercial/1.0.0/
// Distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND.

use crate::pos_tagger::{POSPrediction, POSTag};
use crate::vocab::{
    f8::f8,
    {Pronoun, VocabDatabase},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::ops::Range;

/// Represents a token with linguistic properties, including word, part-of-speech, categories, pronoun details, and scoring information.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Token {
    pub word: String,
    #[serde(skip)]
    pub index: i32,
    pub stem: i32,
    pub potential_stem: Vec<i32>,
    pub is_name: bool,
    #[serde(skip)]
    pub token_type: TokenType,
    #[serde(skip)]
    pub is_possessive: bool,
    #[serde(skip)]
    pub is_negative: bool,
    pub pos: POSTag,
    #[serde(skip)]
    pub pos_prediction: POSPrediction,
    #[serde(skip)]
    pub potential_pos: Vec<POSTag>,
    pub categories: Vec<i16>,
    pub ner: Vec<i16>,
    pub synonyms: Vec<i32>,
    pub hypernyms: Vec<i32>,
    pub hyponyms: Vec<i32>,
    pub classification_scores: HashMap<i8, f8>,
    pub pronoun: Option<Pronoun>,
    #[serde(skip)]
    pub antecedent: Option<String>,
    #[serde(skip)]
    pub inner_word: String,
    #[serde(skip)]
    pub inner_value: String,
    #[serde(skip)]
    pub inner_unit: String,
}

/// Defines the type of a token, which can be a word, prefix, or suffix.
#[derive(Default, Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
pub enum TokenType {
    #[default]
    word,
    prefix,
    suffix,
}

impl Token {
    /// Creates a new Token from a word using the vocabulary database, initializing its properties.
    pub fn new(query_word: &str, vocab: &VocabDatabase) -> Token {
        if query_word.is_empty() {
            return Self::default();
        }

        // Get word lookup table
        let (word, lookup) = match vocab.lookup_word(query_word) {
            Some(r) => r,
            None => return Self::unknown(query_word),
        };
        let (_, token_id) = lookup.iter().next().unwrap();

        // Get oken by id
        let mut token = Self::from_id(*token_id, vocab);
        token.word = word;
        token.token_type = TokenType::word;
        token.potential_pos = lookup.keys().copied().collect();

        token
    }

    /// Creates a prefix Token from a word using the vocabulary database.
    pub fn prefix(word: &str, vocab: &VocabDatabase) -> Token {
        let mut token = Self::new(word, vocab);
        token.token_type = TokenType::prefix;
        token
    }

    /// Creates a suffix Token from a word using the vocabulary database.
    pub fn suffix(word: &str, vocab: &VocabDatabase) -> Token {
        let mut token = Self::new(word, vocab);
        token.token_type = TokenType::suffix;
        token
    }

    /// Creates a numeric Token with the specified word, setting inner word and value.
    pub fn numeric(word: &str, vocab: &VocabDatabase) -> Token {
        let mut token = Self::new("|num|", vocab);
        token.inner_word = word.to_string();
        token.inner_value = word.to_string();
        token
    }

    /// Creates a special Token for system tags, with specified word, tag, value, and unit.
    pub fn special(word: &str, tag: &str, value: &str, unit: &str, vocab: &VocabDatabase) -> Token {
        let mut token = Self::new(tag, vocab);
        token.inner_word = word.to_string();
        token.inner_value = value.to_string();
        token.inner_unit = unit.to_string();
        token
    }

    /// Creates an unknown Token with the specified word and default properties.
    pub fn unknown(word: &str) -> Token {
        Self {
            word: word.to_string(),
            ..Default::default()
        }
    }

    /// Creates a Token from a token ID using the vocabulary database, setting its index.
    pub fn from_id(token_id: i32, vocab: &VocabDatabase) -> Token {
        let mut token = match vocab.words.id2token.get(&token_id) {
            Some(r) => r.clone(),
            None => Self::default(),
        };
        token.index = token_id;

        token
    }

    /// Updates the POS tag of the Token, returning a new Token if the tag is valid in the vocabulary.
    pub fn update_pos(&self, pos_code: POSTag, vocab: &VocabDatabase) -> Option<Token> {
        // Get map
        let index_map = vocab.words.wordlist.get(&self.word)?;

        // Get token id
        let index = index_map.get(&pos_code)?;

        // Return token
        let token = Self::from_id(*index, vocab);
        Some(token)
    }

    /// Forces the Token to a verb POS tag if possible, returning a new Token or None if no verb tag is available.
    pub fn force_verb(&self, vocab: &VocabDatabase) -> Option<Token> {
        if self.is_verb() {
            return None;
        }

        for code in self.potential_pos.iter() {
            if !code.to_str().starts_with("V") {
                continue;
            }
            return self.update_pos(*code, vocab);
        }

        None
    }

    /// Checks if the Token has a category within the specified range.
    pub fn has_category(&self, category_range: &Range<i16>) -> bool {
        self.categories.iter().any(|&x| category_range.contains(&x))
    }

    /// Checks if the Token has a named entity recognition (NER) category within the specified range.
    pub fn has_ner(&self, category_range: &Range<i16>) -> bool {
        self.ner.iter().any(|&x| category_range.contains(&x))
    }

    /// Checks if the Token is a noun (starts with 'N' or is SYS).
    pub fn is_noun(&self) -> bool {
        self.pos.to_str().starts_with("N") || self.pos == POSTag::SYS
    }

    /// Checks if the Token is a verb (starts with 'V').
    pub fn is_verb(&self) -> bool {
        self.pos.to_str().starts_with("V")
    }

    /// Checks if the Token is a base verb (VB or VBG).
    pub fn is_base_verb(&self) -> bool {
        ["VB", "VBG"].contains(&self.pos.to_str().as_str())
    }

    /// Checks if the Token is a past verb (VBD, VBN, or VHP).
    pub fn is_past_verb(&self) -> bool {
        ["VBD", "VBN", "VHP"].contains(&self.pos.to_str().as_str())
    }

    /// Checks if the Token is a present verb (VB, VBG, VBZ, VH, or VHZ).
    pub fn is_present_verb(&self) -> bool {
        ["VB", "VBG", "VBZ", "VH", "VHZ"].contains(&self.pos.to_str().as_str())
    }

    /// Checks if the Token is a future verb (VF, VFG, or VHF).
    pub fn is_future_verb(&self) -> bool {
        ["VF", "VFG", "VHF"].contains(&self.pos.to_str().as_str())
    }

    /// Checks if the Token is an adjective (starts with 'JJ').
    pub fn is_adjective(&self) -> bool {
        self.pos.to_str().starts_with("JJ")
    }

    /// Checks if the Token is an adverb (starts with 'RB').
    pub fn is_adverb(&self) -> bool {
        self.pos.to_str().starts_with("RB")
    }

    /// Checks if the Token is a named entity (starts with 'NNP').
    pub fn is_named_entity(&self) -> bool {
        self.pos.to_str().starts_with("NNP")
    }

    /// Checks if the Token is an n-gram (MWE).
    pub fn is_ngram(&self) -> bool {
        self.pos == POSTag::MWE
    }

    /// Checks if the Token is a conjunction (starts with 'C').
    pub fn is_conjunction(&self) -> bool {
        self.pos.to_str().starts_with("C")
    }

    /// Checks if the Token is a determiner (DT).
    pub fn is_determiner(&self) -> bool {
        self.pos == POSTag::DT
    }

    /// Checks if the Token is a pronoun (PR or PRP).
    pub fn is_pronoun(&self) -> bool {
        self.pos == POSTag::PR || self.pos == POSTag::PRP
    }

    /// Checks if the Token is a modal verb (MD).
    pub fn is_modal_verb(&self) -> bool {
        self.pos == POSTag::MD
    }

    /// Checks if the Token is a preposition (IN).
    pub fn is_preposition(&self) -> bool {
        self.pos == POSTag::IN
    }

    /// Checks if the Token is a sentence stopper (SS).
    pub fn is_sentence_stopper(&self) -> bool {
        self.pos == POSTag::SS
    }

    /// Check if the token is a punctuation mark
    pub fn is_punctuation(&self) -> bool {
        self.pos == POSTag::SS || self.pos == POSTag::PUNC
    }

    /// Checks if the Token is a potential phrase splitter (PUNC).
    pub fn is_phrase_splitter(&self) -> bool {
        self.pos == POSTag::PUNC
    }

    /// Retrieves the category vectors for the Token from the vocabulary database.
    pub fn get_category_vec(&self, vocab: &VocabDatabase) -> Vec<Vec<i16>> {
        let mut res: Vec<Vec<i16>> = Vec::new();
        for category_id in self.categories.iter() {
            let cat = match vocab.categories.get(category_id) {
                Some(r) => r,
                None => continue,
            };
            res.push(cat.fqn.clone());
        }

        res
    }

    /// Calculates the semantic distance between two Tokens based on their category vectors.
    pub fn get_distance(&self, token2: &Token, vocab: &VocabDatabase) -> f32 {
        // Get category vectors
        let token1_categories = self.get_category_vec(vocab);
        let token2_categories = token2.get_category_vec(vocab);

        // Initialize
        let mut total_score = 0.0;
        let mut comparisons = 0;

        // Go through categories, calculate distance / score
        for cat1 in token1_categories.iter() {
            for cat2 in token2_categories.iter() {
                let depth = self.get_common_category_depth(cat1, cat2);
                //let depth = 1;
                total_score += depth as f32;
                comparisons += 1;
            }
        }

        if comparisons > 0 {
            total_score / comparisons as f32
        } else {
            0.0
        }
    }

    /// Calculates the common depth between two category paths.
    fn get_common_category_depth(&self, path1: &[i16], path2: &[i16]) -> usize {
        let mut depth = 0;
        for (p1, p2) in path1.iter().zip(path2.iter()) {
            if p1 == p2 {
                depth += 1;
            } else {
                break;
            }
        }

        depth
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(antecedent) = &self.antecedent {
            write!(
                f,
                "{} ({}), antecedent: {}",
                self.word, self.pos, antecedent
            )
        } else if self.pos == POSTag::SYS && !self.inner_word.is_empty() {
            write!(
                f,
                "{} ({}), inner word: {}, value: {}, unit{}",
                self.word, self.pos, self.inner_word, self.inner_value, self.inner_unit
            )
        } else {
            write!(f, "{} ({})", self.word, self.pos)
        }
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::word => write!(f, "word"),
            TokenType::prefix => write!(f, "prefix"),
            TokenType::suffix => write!(f, "suffix"),
        }
    }
}
