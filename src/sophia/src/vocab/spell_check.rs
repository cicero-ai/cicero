// Copyright 2025 Aquila Labs of Alberta, Canada <matt@cicero.sh>
// Licensed under the PolyForm Noncommercial License 1.0.0
// Commercial use requires a separate license: https://cicero.sh/sophia/
// License text: https://polyformproject.org/licenses/noncommercial/1.0.0/
// Distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND.

use std::fmt;
use super::VocabDatabase;
use crate::pos_tagger::{POSTag, POSSuffix, POSPrefix};
use crate::tokenizer::Token;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const MAX_FREQUENCY: usize = 3;
const FREQUENCY_WEIGHT: f32 = 0.40;
const DISTANCE_WEIGHT: f32 = 0.85;
const TAG_BEFORE_WEIGHT: f32 = 0.55;
const WORD_BEFORE_WEIGHT: f32 = 0.65;
const SUFFIX_BONUS: f32 = 0.75;
const PREFIX_BONUS: f32 = 0.75;
const DOUBLE_LETTER_BONUS: f32 = 0.35;

/// Represents the automated spell checker, namely the various cohorts
/// that are based on POS / word length and used to minimize the search space of possible corrections.
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct SpellChecker {
    pub cohorts: HashMap<SpellCheckerCohort, Vec<SpellCheckerEntry>>,
}

/// Individual entry for a candidate, stores 
/// preceeding tag / word frequency for weighted scoring.
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct SpellCheckerEntry {
    pub word_index: i32,
    pub tag_before: Vec<POSTag>,
    pub word_before: Vec<i32>
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SpellCheckerCohort {
    pub pos: SpellCheckerCohortPOS,
    pub length: SpellCheckerCohortSize,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum SpellCheckerCohortPOS {
    noun,
    verb,
    adverb,
    adjective,
    entity,
    other,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum SpellCheckerCohortSize {
    short,  // <= 4 chars
    short_medium,  // 5 or 6 chars
    medium, // 7 or 8 chars
    medium_long,  // 9 or 10 chars
    long, // 11+ chars
}

/// Candidate spelling correct, used to rank and 
//  score possible corrections.
#[derive(Default, Clone)]
struct Candidate {
    pub token: Token,
    pub item: SpellCheckerEntry,
    pub score: f32,
    pub frequency: usize,
    pub distance: usize,
    pub tag_before: usize,
    pub word_before: usize,
    pub same_suffix: bool,
    pub same_prefix: bool,
    pub has_double_letter: bool
}

impl SpellChecker {
    /// Check word for corrected spelling
    pub fn try_correct(&self, position: usize, tokens: &[Token], vocab: &VocabDatabase) -> Option<Token> {

        // Get candidates
        let mut candidates =  self.get_candidates(&tokens[position], vocab);

        // Look for spelling correction
        for queue in &mut candidates {
            if queue.is_empty() { continue; }

            // Score candidates
            self.score_candidates(queue, position, tokens);

            // Sort candidates
            queue.sort_unstable_by(|a,b| b.score.partial_cmp(&a.score).unwrap());
            return Some(queue[0].token.clone());
        }

        None
    }

    /// Get cohort based on POS tag and length
    fn get_cohorts(&self, token: &Token) -> Vec<SpellCheckerCohort> {

        // Get tags
        let tags = token.pos_prediction.probabilities.iter()
            .filter(|(_, score)| **score >= 0.2)
            .map(|(tag, _)| *tag).collect::<Vec<POSTag>>();

        // Go through tags
        let cohorts: Vec<SpellCheckerCohort> = tags.iter().flat_map(|tag| {
            let pos = SpellCheckerCohortPOS::from(*tag);
            let sizes = SpellCheckerCohortSize::get_sizes(token.word.len());

            sizes.iter().map(|length| {
                SpellCheckerCohort {
                    pos: pos.clone(),
                    length: length.clone()
                }
            }).collect::<Vec<SpellCheckerCohort>>()

        })  .collect();

        cohorts
    }

        // Get initial candidates, sorted by distance
    fn get_candidates(&self, token: &Token, vocab: &VocabDatabase) -> Vec<Vec<Candidate>> {

        // Get cohorts
        let cohorts = self.get_cohorts(token);
        let mut candidates: Vec<Vec<Candidate>> = vec![vec![]; 4];
        let word = token.word.to_lowercase();

        // Go through cohorts
        for cohort in cohorts.iter() {

            let search = match self.cohorts.get(cohort) {
                Some(r) => r,
                None => continue
            };

            // Initialize variables
            let mut frequency = MAX_FREQUENCY;
            let freq_interval = search.len() / 3;

            // Gather candidates
            for (x, item) in search.iter().enumerate() {
                if x > 0 && x % freq_interval == 0 {
                    frequency -= 1;
                }
                let s_token = vocab.from_int(item.word_index);

                // GEt distance
                let lev_distance = self.levenshtein(&word, &s_token.word);
                let distance = candidates.len().saturating_sub(lev_distance);
                if distance > 0 && lev_distance > 0 {
                    candidates[lev_distance-1].push( Candidate ::new(frequency, distance, & s_token, item) );
                }

            }
        }

        candidates
    }

    // Score candidates
    fn score_candidates(&self, candidates: &mut [Candidate], position: usize, tokens: &[Token]) {

        // Iterate through candidates
        for cand in candidates.iter_mut() {

            // Get preceding tag and word score
            if position > 0 {
                if let Some(idx) = cand.item.tag_before.iter().position(|&tag| tag == tokens[position-1].pos) {
                    cand.tag_before = self.get_frequency_idx(idx, cand.item.tag_before.len());
                }

                if let Some(idx) = cand.item.word_before.iter().position(|&w_idx| w_idx == tokens[position-1].index) {
                    cand.word_before = self.get_frequency_idx(idx, cand.item.word_before.len());
                }
            }

            // Check suffix
            if let Ok(suffix) = POSSuffix::try_from(&tokens[position])
                && let Ok(chk_suffix) = POSSuffix::try_from(&cand.token) {
                    cand.same_suffix = suffix == chk_suffix;
                }

            // Check prefix
            if let Ok(prefix) = POSPrefix::try_from(&tokens[position])
                && let Ok(chk_prefix) = POSPrefix::try_from(&cand.token) {
                    cand.same_prefix = prefix == chk_prefix;
                }

            // Check double letter
            cand.has_double_letter = self.check_double_letter(&tokens[position].word.to_lowercase(), &cand.token.word);

            // Score candidate
            cand.score = cand.calculate_score();
        }

    }

    // Check whether or not word has double letter typo
    fn check_double_letter(&self, word: &str, candidate_word: &str) -> bool {

        let letters: Vec<char> = word.chars().collect();
        for (x, char) in letters[1..].iter().enumerate() {
            if *char == letters[x] {

                // Check if candidate has double letter
                let chk = format!("{}{}", char, char);
                if !candidate_word.contains(&chk) {
                    return true;
                }
            }
        }

        false
    }

    /// Get frequecny based score
    fn get_frequency_idx(&self, idx: usize, total: usize) -> usize {
        let interval = total / MAX_FREQUENCY;
        if idx == 0 || interval == 0 {
            MAX_FREQUENCY
        } else {
            MAX_FREQUENCY - (idx / interval)
        }
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

impl Candidate {
    pub fn new(frequency: usize, distance: usize, token: &Token, item: &SpellCheckerEntry) -> Self {
        Self {
            token: token.clone(),
            item: item.clone(),
            frequency, distance,
            ..Default::default()
        }
    }

    /// Score the candidate
    pub fn calculate_score(&mut self) -> f32 {
        let mut score = (self.frequency as f32 * FREQUENCY_WEIGHT) + (self.distance as f32 * DISTANCE_WEIGHT);
        score += (self.tag_before as f32 * TAG_BEFORE_WEIGHT) + (self.word_before as f32 * WORD_BEFORE_WEIGHT);

        if self.same_suffix { score += SUFFIX_BONUS; }
        if self.same_prefix { score += PREFIX_BONUS; }
        if self.has_double_letter { score += DOUBLE_LETTER_BONUS; }

        score
    }
}

impl fmt::Debug for Candidate {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "word {} pos {} score {:.2} frequency {} distance {} tag_before {} word before {} suffix {} prefix {}", self.token.word, self.token.pos, self.score, self.frequency, self.distance, self.tag_before, self.word_before, self.same_suffix, self.same_prefix)
    }
}

impl From<POSTag> for SpellCheckerCohortPOS {
    fn from(tag: POSTag) -> Self {
        match tag {
            t if t.is_named_entity() => Self::entity,
            t if t.is_noun() => Self::noun,
            t if t.is_verb() => Self::verb,
            t if t.is_adverb() => Self::adverb,
            t if t.is_adjective() => Self::adjective,
            _ => Self::other
        }
    }
}

impl From<usize> for SpellCheckerCohortSize {
    fn from(length: usize) -> Self {
        match length {
            len if len <= 4 => Self::short,
            len if len <= 6 => Self::short_medium,
            len if len <= 8 => Self::medium,
            len if len <= 10 => Self::medium_long,
            _ => Self::long
        }
    }
}

impl SpellCheckerCohortSize {
    pub fn get_sizes(length: usize) -> Vec<Self> {
        match length {
            len if len <= 3 => vec![Self::short],
        len if len <= 5 => vec![Self::short, Self::short_medium],
        len if len <= 7 => vec![Self::short_medium, Self::medium],
        len if len <= 11 => vec![Self::medium, Self::medium_long, Self::long],
            _ => vec![Self::medium_long, Self::long]
        }
    }
}

