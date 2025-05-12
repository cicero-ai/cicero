
// Copyright 2025 Aquila Labs of Alberta, Canada <matt@cicero.sh>
// Licensed under the Functional Source License, Version 1.1 (FSL-1.1)
// See the full license at: https://cicero.sh/license.txt
// Distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND.

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use super::POSTag;

/// A trie structure for exact match POS tagging, mapping character sequences to tags.
#[derive(Default, Serialize, Deserialize, Clone)]
pub struct POSTaggerExactMatchTrie {
    pub score: Option<POSTag>,
    pub children: HashMap<POSTag, Box<POSTaggerExactMatchTrie>>,
}

impl POSTaggerExactMatchTrie {

    /// Creates a new exact match an optional expected verb POS tag and empty children.
    pub fn new(score: Option<POSTag>) -> Self {
        Self {
            score,
            children: HashMap::new(),
        }
    }

    /// Inserts a entry into the trie,
    pub fn insert(&mut self, context: &Vec<POSTag>, res_tag: POSTag) {
        let mut current = self;
        for tag in context.iter() {
            current = current.children.entry(*tag).or_default();
            if current.score.is_some() { return; }
        }

        current.score = Some(res_tag);
    }

    /// Lookup context
    pub fn lookup(&self, context: &Vec<POSTag>) -> Option<POSTag> {
        let mut child = self;
        let mut x = 0;

        while let Some(next_node) = child.children.get(&context[x]) {
            if let Some(score) = next_node.score {
                return Some(score);
            }
            if x >= context.len() { break; }

            child = next_node;
            x += 1;
        }

        None
    }

}

