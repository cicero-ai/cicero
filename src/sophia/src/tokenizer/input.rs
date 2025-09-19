// Copyright 2025 Aquila Labs of Alberta, Canada <matt@cicero.sh>
// Licensed under the PolyForm Noncommercial License 1.0.0
// Commercial use requires a separate license: https://cicero.sh/sophia/
// License text: https://polyformproject.org/licenses/noncommercial/1.0.0/
// Distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND.

use super::Token;

/// Represents the tokenized output of input text, including tokens, multi-word entities (MWEs), and iteration state.
#[derive(Default, Clone)]
pub struct TokenizedInput {
    pub original: String,
    pub tokens: Vec<Token>,
    pub mwe: Vec<MWE>,
    pub mwe_scoring: Vec<MWE>,
    position: usize,
    filter_mwe: bool,
    filter_mwe_scoring: bool,
    filter_stopwords: bool,
}

/// Represents a multi-word entity (MWE) with a position and an optional associated token.
#[derive(Clone)]
pub struct MWE {
    pub position: usize,
    pub token: Option<Token>,
}

impl TokenizedInput {
    /// Creates a new TokenizedInput instance with the provided original text and empty token/MWE lists.
    pub fn new(original: &str) -> Self {
        Self {
            original: original.to_string(),
            tokens: Vec::new(),
            mwe: Vec::new(),
            mwe_scoring: Vec::new(),
            position: 0,
            filter_mwe: false,
            filter_mwe_scoring: false,
            filter_stopwords: false,
        }
    }

    /// Returns a new TokenizedInput configured to iterate over individual tokens.
    pub fn iter(&self) -> Self {
        let mut c = self.clone();
        c.filter_mwe = false;
        c.position = 0;
        c
    }

    /// Returns a new TokenizedInput configured to iterate over MWEs.
    pub fn mwe(&self) -> Self {
        let mut c = self.clone();
        c.filter_mwe = true;
        c.position = 0;
        c
    }

    /// Returns a new TokenizedInput configured to iterate over MWE scoring tokens.
    pub fn mwe_scoring(&self) -> Self {
        let mut c = self.clone();
        c.filter_mwe_scoring = true;
        c.position = 0;
        c
    }

    /// Configures the TokenizedInput to filter out stopwords during iteration.
    pub fn remove_stop_words(mut self) -> Self {
        self.filter_stopwords = true;
        self
    }

    /// Configures the TokenizedInput to include stopwords during iteration.
    pub fn add_stop_words(mut self) -> Self {
        self.filter_stopwords = false;
        self
    }

    /// Retrieves the next MWE token, either from the MWE's token or the token at the MWE's position.
    fn next_mwe(&mut self) -> Option<Token> {
        if self.position >= self.mwe.len() {
            return None;
        }
        let mwe = self.mwe.get(self.position).unwrap();
        self.position += 1;

        let token = match mwe.token.clone() {
            Some(r) => r,
            None => self.tokens.get(mwe.position).unwrap().clone(),
        };

        Some(token)
    }

    /// Retrieves the next MWE scoring token, either from the MWE scoring's token or the token at its position.
    fn next_mwe_scoring(&mut self) -> Option<Token> {
        if self.position >= self.mwe_scoring.len() {
            return None;
        }
        let mwe = self.mwe_scoring.get(self.position).unwrap();
        self.position += 1;

        let token = match mwe.token.clone() {
            Some(r) => r,
            None => self.tokens.get(mwe.position).unwrap().clone(),
        };

        Some(token)
    }
}

impl std::ops::Index<usize> for TokenizedInput {
    type Output = Token;

    /// Provides read-only indexing into the token vector by position.
    fn index(&self, index: usize) -> &Self::Output {
        &self.tokens[index]
    }
}

impl std::ops::IndexMut<usize> for TokenizedInput {
    /// Provides mutable indexing into the token vector by position.
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.tokens[index]
    }
}

impl Iterator for TokenizedInput {
    /// Advances the iterator, returning the next token based on the current filter (MWE, MWE scoring, or individual tokens).
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        // MWE
        if self.filter_mwe {
            return self.next_mwe();
        } else if self.filter_mwe_scoring {
            return self.next_mwe_scoring();
        }

        if self.position >= self.tokens.len() {
            return None;
        }
        let token = self.tokens.get(self.position).unwrap();
        self.position += 1;

        Some(token.clone())
    }
}
