// Copyright 2025 Aquila Labs of Alberta, Canada <matt@cicero.sh>
// Licensed under the PolyForm Noncommercial License 1.0.0
// Commercial use requires a separate license: https://cicero.sh/sophia/
// License text: https://polyformproject.org/licenses/noncommercial/1.0.0/
// Distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND.

use super::{POSPrediction, POSPredictionMethod, POSTag, TokenKey};
use crate::tokenizer::Token;
use crate::vocab::{Capitalization, VocabMWE};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;

pub const TOTAL_TAGS: usize = 47;

#[derive(Default, Serialize, Deserialize)]
#[serde(bound = "S: Default + Clone + Eq + PartialEq + Hash + Serialize + for<'a> Deserialize<'a>")]
pub struct HMM<S> {
    pub vocab_size: f32,
    pub initial_probs: Vec<f32>,
    pub transmition_probs: Vec<Vec<f32>>,
    pub emission_probs: Vec<HashMap<S, f32>>,
    pub smoothing: f64,
}

#[derive(Debug)]
struct Probability {
    pub deterministic_tag_idx: usize,
    pub viterbi: Vec<f32>,
    pub backpointer: Vec<usize>,
}

impl<S> HMM<S>
where
    S: Default + Clone + Eq + PartialEq + Hash + Serialize + for<'a> Deserialize<'a>,
    Token: TokenKey<S>,
{
    pub fn new() -> Self {
        Self {
            vocab_size: 0.0,
            initial_probs: vec![0.0; TOTAL_TAGS],
            transmition_probs: vec![vec![0.0; TOTAL_TAGS]; TOTAL_TAGS],
            emission_probs: vec![HashMap::new(); TOTAL_TAGS],
            smoothing: 1.0,
        }
    }

    /// Apply hmm model to vector of tokens
    pub fn apply(&self, tokens: &mut [Token]) {
        let mut start_pos = 0;
        let mut end_pos: usize;
        loop {
            // Get end position
            end_pos = match tokens[start_pos..].iter().position(|token| token.pos == POSTag::SS) {
                Some(r) => r + start_pos + 1,
                None => tokens.len() - 1,
            };
            if start_pos >= end_pos {
                break;
            }

            // Apply viterbi
            self.viterbi_decode(start_pos, end_pos, tokens);
            start_pos = end_pos;
            if start_pos >= tokens.len() - 1 {
                break;
            }
        }
    }

    /// Predict tags for a sentence
    fn viterbi_decode(&self, start_pos: usize, end_pos: usize, tokens: &mut [Token]) {
        // Go through tokens
        let mut results: Vec<Probability> = Vec::new();
        for (offset, token) in tokens[start_pos..end_pos].iter().enumerate() {
            let position = offset + start_pos;

            // Initial token
            if offset == 0 {
                let tag_indices = if token.potential_pos.len() > 1 {
                    token
                        .potential_pos
                        .iter()
                        .filter(|&tag| *tag != POSTag::FW)
                        .map(|tag| tag.to_u8() as usize)
                        .collect::<Vec<usize>>()
                } else if token.pos == POSTag::FW {
                    (1..47).filter(|x| *x != 6).collect::<Vec<usize>>()
                } else {
                    vec![token.pos.to_u8() as usize]
                };

                // Instantiate probability
                let current_tag_idx = if tag_indices.len() == 1 {
                    tag_indices[0]
                } else {
                    0
                };
                let mut probs = Probability::new(current_tag_idx);

                for tag_idx in tag_indices {
                    probs.viterbi[tag_idx] =
                        self.initial_probs[tag_idx] + self.get_emission_prob(tag_idx, token);
                }

                results.push(probs);
                continue;
            }

            // Forward pass
            let probs = self.calculate_viterbi(position, &results, tokens);
            results.push(probs);
        }

        // Find best final state
        let last_idx = results.len() - 1;
        let mut best_final_state = 0;
        let mut best_score = results[last_idx].viterbi[0];

        for tag_idx in 1..TOTAL_TAGS {
            if tag_idx == 6 {
                continue;
            }

            if results[last_idx].viterbi[tag_idx] > best_score {
                best_score = results[last_idx].viterbi[tag_idx];
                best_final_state = tag_idx;
            }
        }

        // Backtrack to find best path
        let mut path = vec![0; results.len()];
        path[last_idx] = best_final_state;
        for idx in (0..results.len() - 1).rev() {
            path[idx] = results[idx + 1].backpointer[path[idx + 1]];
        }

        // Update tokens with new POS tags
        let (mut is_initial, mut in_nnp) = (true, false);
        for (offset, tag_idx) in path.iter().enumerate() {
            let position = offset + start_pos;

            if tokens[position].potential_pos.len() < 2 && tokens[position].index > 0 {
                tokens[position].pos_prediction.confidence = 1.0;
                tokens[position].pos_prediction.tag = tokens[position].pos;
                tokens[position].pos_prediction.prev_tag = tokens[position].pos;
            } else {
                let tag = POSTag::from_u8(*tag_idx as u8);
                let confidence = self.get_confidence_score(position, offset, &results, tokens);

                tokens[position].pos_prediction = POSPrediction::new(
                    POSPredictionMethod::hmm,
                    &tokens[position].word,
                    tokens[position].pos,
                    tag,
                    confidence,
                    &HashMap::new(),
                    &[],
                );
                tokens[position].pos = tag;
            }

            // Check for named entity
            let tag = tokens[position].pos;
            if tag == POSTag::NN
                && VocabMWE::classify_capitalization(&tokens[position].word)
                    != Capitalization::lower
                && !is_initial
            {
                //tokens[position].pos = POSTag::NNP;
                in_nnp = true;
            } else if tag == POSTag::NN && in_nnp {
                //tokens[position].pos = POSTag::NNP;
            } else {
                in_nnp = false;
            }
            is_initial = tokens[position].pos == POSTag::SS;
        }
    }

    // Calculate the viterbi for a single token.
    fn calculate_viterbi(
        &self,
        position: usize,
        results: &[Probability],
        tokens: &[Token],
    ) -> Probability {
        // Instantiate probability
        let token = &tokens[position];
        let deterministic_tag_idx = if token.potential_pos.len() > 1 || token.pos == POSTag::FW {
            0
        } else {
            token.pos.to_u8() as usize
        };
        let mut probs = Probability::new(deterministic_tag_idx);

        // Initialize
        let prev_probs = &results.last().unwrap();

        // Get tag indices
        let tag_indices = if token.potential_pos.len() > 1 {
            token
                .potential_pos
                .iter()
                .filter(|&tag| *tag != POSTag::FW)
                .map(|tag| tag.to_u8() as usize)
                .collect::<Vec<usize>>()
        } else if token.pos == POSTag::FW {
            (1..47).filter(|x| *x != 6).collect::<Vec<usize>>()
        } else {
            vec![token.pos.to_u8() as usize]
        };

        // Get previous tag indices
        let prev_tag_indices = if tokens[position - 1].potential_pos.len() > 1 {
            tokens[position - 1]
                .potential_pos
                .iter()
                .filter(|&tag| *tag != POSTag::FW)
                .map(|tag| tag.to_u8() as usize)
                .collect::<Vec<usize>>()
        } else if tokens[position - 1].pos == POSTag::FW || prev_probs.deterministic_tag_idx == 6 {
            (1..47).filter(|x| *x != 6).collect::<Vec<usize>>()
        } else {
            vec![prev_probs.deterministic_tag_idx]
        };

        // Calculate scores
        for tag_idx in tag_indices.iter() {
            let emission_prob = self.get_emission_prob(*tag_idx, token);

            for prev_tag_idx in prev_tag_indices.iter() {
                let score = prev_probs.viterbi[*prev_tag_idx]
                    + self.transmition_probs[*prev_tag_idx][*tag_idx]
                    + emission_prob;
                if score > probs.viterbi[*tag_idx] {
                    probs.viterbi[*tag_idx] = score;
                    probs.backpointer[*tag_idx] = *prev_tag_idx;
                }
            }
        }

        probs
    }

    /// Get emission probabilities
    fn get_emission_prob(&self, tag_idx: usize, token: &Token) -> f32 {
        match self.emission_probs[tag_idx].get(&token.get_key()) {
            Some(&prob) => prob,
            None => {
                let tag_vocab_size = self.emission_probs[tag_idx].len() as f32;
                (self.smoothing as f32 / (tag_vocab_size + self.vocab_size * self.smoothing as f32))
                    .ln()
            }
        }
    }

    /// Returns a value between 0.0 and 1.0, where 1.0 means completely certain
    fn get_confidence_score(
        &self,
        position: usize,
        offset: usize,
        results: &[Probability],
        tokens: &[Token],
    ) -> f32 {
        let token = &tokens[position];
        let prob_result = &results[offset];

        let tag_indices: Vec<usize> = token
            .potential_pos
            .iter()
            .filter(|&tag| *tag != POSTag::FW)
            .map(|tag| tag.to_u8() as usize)
            .collect();

        // Get scores for all possible tags at this position
        let mut scores: Vec<f32> = tag_indices
            .iter()
            .map(|&idx| prob_result.viterbi[idx])
            .filter(|&score| score != f32::NEG_INFINITY)
            .collect();

        if scores.len() <= 1 {
            return 1.0;
        }

        // Sort scores in descending order
        scores.sort_by(|a, b| b.partial_cmp(a).unwrap());

        // Convert log probabilities to actual probabilities
        let max_score = scores[0];
        let prob_scores: Vec<f32> = scores.iter().map(|&score| (score - max_score).exp()).collect();

        let total_prob: f32 = prob_scores.iter().sum();
        let normalized_probs: Vec<f32> =
            prob_scores.iter().map(|&prob| prob / total_prob).collect();

        // Confidence is the probability of the best choice
        normalized_probs[0]
    }
}

impl Probability {
    pub fn new(deterministic_tag_idx: usize) -> Self {
        Self {
            deterministic_tag_idx,
            viterbi: vec![f32::NEG_INFINITY; TOTAL_TAGS],
            backpointer: vec![0; TOTAL_TAGS],
        }
    }
}
