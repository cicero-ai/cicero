// Copyright 2025 Aquila Labs of Alberta, Canada <matt@cicero.sh>
// Licensed under the PolyForm Noncommercial License 1.0.0
// Commercial use requires a separate license: https://cicero.sh/sophia/
// License text: https://polyformproject.org/licenses/noncommercial/1.0.0/
// Distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND.

use super::{Buffer, Phrase};
use crate::tokenizer::Token;
use std::collections::HashMap;

/// Represents the result of interpreting input, containing classification scores, tokens, multi-word expressions (MWE), and phrases.
pub struct Interpretation {
    pub scores: HashMap<i8, f32>,
    pub tokens: Vec<Token>,
    pub mwe: Vec<Token>,
    pub phrases: Vec<Phrase>,
}

impl Interpretation {
    /// Adds a phrase to the interpretation, checking for enclosed character phrases in the buffer before appending.
    pub fn push_phrase(&mut self, phrase: Phrase, buffer: &mut Buffer) {
        // Combine enclosed phrases, if needed
        buffer.enclosed_chars.is_empty();

        self.phrases.push(phrase);
    }
}
