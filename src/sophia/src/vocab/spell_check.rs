use super::VocabWordDatabase;
use crate::pos_tagger::POSTag;
use crate::tokenizer::Token;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents the automated spell checker, namely the various cohorts
/// that are based on POS / word length and used to minimize the search space of possible corrections.
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct SpellChecker {
    pub cohorts: HashMap<SpellCheckerCohort, Vec<i32>>,
}

#[derive(Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SpellCheckerCohort {
    pub pos: SpellCheckerCohortPOS,
    pub length: SpellCheckerCohortSize,
}

#[derive(Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum SpellCheckerCohortPOS {
    noun,
    verb,
    adverb,
    adjective,
    entity,
    other,
}

#[derive(Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum SpellCheckerCohortSize {
    short,        // 1-4 chars, typo: 1-5 chars
    short_medium, // 3-7, typo: 6-8
    medium,       // 6-10 chars: 9-11
    long,         // 9+ chars, typo: 12+
}

impl SpellChecker {
    /// Check word for corrected spelling
    pub fn try_correct(
        &self,
        word: &str,
        tag: POSTag,
        worddb: &VocabWordDatabase,
    ) -> Option<Token> {
        let length = match word.len() {
            len if len < 6 => SpellCheckerCohortSize::short,
            len if len < 9 => SpellCheckerCohortSize::short_medium,
            len if len < 12 => SpellCheckerCohortSize::medium,
            _ => SpellCheckerCohortSize::long,
        };

        // Get pos
        let pos = self.get_pos(tag);
        let cohort = SpellCheckerCohort { pos, length };
        let (mut min_distance, mut min_token) = (6, Token::default());

        // Go through words
        let search = self.cohorts.get(&cohort)?;
        for index in search.iter() {
            let chk_token = worddb.id2token.get(index).unwrap();
            let distance = self.levenshtein(&word, &chk_token.word);
            if distance <= 2 || distance < min_distance {
                min_distance = distance;
                min_token = chk_token.clone();
            }
            if min_distance <= 2 {
                break;
            }
        }

        if min_token.pos == POSTag::FW {
            None
        } else {
            Some(min_token.clone())
        }
    }

    /// Get part of speech based on POS tag
    fn get_pos(&self, tag: POSTag) -> SpellCheckerCohortPOS {
        let mut pos = SpellCheckerCohortPOS::other;
        if tag.is_noun() {
            pos = SpellCheckerCohortPOS::noun;
        } else if tag.is_verb() {
            pos = SpellCheckerCohortPOS::verb;
        } else if tag.is_adverb() {
            pos = SpellCheckerCohortPOS::adverb;
        } else if tag.is_adjective() {
            pos = SpellCheckerCohortPOS::adjective;
        } else if tag.is_named_entity() {
            pos = SpellCheckerCohortPOS::entity;
        }

        pos
    }

    /// Calculate levenshtein distance
    fn levenshtein(&self, s1: &str, s2: &str) -> usize {
        let len1 = s1.len();
        let len2 = s2.len();

        if len1 == 0 {
            return len2;
        }
        if len2 == 0 {
            return len1;
        }

        let mut prev_row = vec![0; len2 + 1];
        let mut curr_row = vec![0; len2 + 1];

        for j in 0..=len2 {
            prev_row[j] = j;
        }

        for (i, c1) in s1.chars().enumerate() {
            curr_row[0] = i + 1;

            for (j, c2) in s2.chars().enumerate() {
                let cost = if c1 == c2 { 0 } else { 1 };
                curr_row[j + 1] = std::cmp::min(
                    std::cmp::min(prev_row[j + 1] + 1, curr_row[j] + 1),
                    prev_row[j] + cost,
                );
            }

            std::mem::swap(&mut prev_row, &mut curr_row);
        }

        prev_row[len2]
    }
}
