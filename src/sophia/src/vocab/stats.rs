// Copyright 2025 Aquila Labs of Alberta, Canada <matt@cicero.sh>
// Licensed under the Functional Source License, Version 1.1 (FSL-1.1)
// See the full license at: https://cicero.sh/license.txt
// Distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND.

use super::VocabDatabase;
use crate::pos_tagger::POSTag;
use std::collections::HashMap;

/// Populated with basic statistical and meta information regarding the vocabulary data store
/// including number of words, MWEs, ambiguous words, named entities, categories, and so on.
#[derive(Debug, Default)]
pub struct VocabStats {
    pub singular_words: usize,
    pub ambiguous_words: usize,
    pub mwes: usize,
    pub nouns: usize,
    pub verbs: usize,
    pub adverbs: usize,
    pub adjectives: usize,
    pub named_entities: usize,
    pub synonyms: usize,
    pub hypernyms: usize,
    pub hyponyms: usize,
    pub pos_tags: HashMap<POSTag, usize>,
}

impl VocabStats {
    pub fn compile(vocab: &VocabDatabase) -> Self {
        let mut stats = Self::default();

        // GO through wordlist
        for (_, pos_map) in vocab.words.wordlist.iter() {
            // Singular or ambiguous?
            if pos_map.len() > 1 {
                stats.ambiguous_words += 1;
            }

            // POS tags
            for (tag, _) in pos_map.iter() {
                *stats.pos_tags.entry(*tag).or_insert(0) += 1;
            }
        }

        // Go through all tokens
        for (_, token) in vocab.words.id2token.iter() {
            // MWE?
            if token.word.contains(" ") {
                stats.mwes += 1;
            } else {
                stats.singular_words += 1;
            }

            // Counts
            stats.synonyms += token.synonyms.len();
            stats.hypernyms += token.hypernyms.len();
            stats.hyponyms += token.hyponyms.len();

            // Part of speech
            if token.is_noun() {
                stats.nouns += 1;
            } else if token.is_verb() {
                stats.verbs += 1;
            } else if token.is_adverb() {
                stats.adverbs += 1;
            } else if token.is_adjective() {
                stats.adjectives += 1;
            }

            if token.is_named_entity() {
                stats.named_entities += 1;
            }
        }

        stats
    }
}
