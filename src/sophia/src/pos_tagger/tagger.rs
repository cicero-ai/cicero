// Copyright 2025 Aquila Labs of Alberta, Canada <matt@cicero.sh>
// Licensed under the PolyForm Noncommercial License 1.0.0
// Commercial use requires a separate license: https://cicero.sh/sophia/
// License text: https://polyformproject.org/licenses/noncommercial/1.0.0/
// Distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND.

use super::{HMM, POSModel, POSModelInterface, POSTag, POSTagModelRepo};
use crate::tokenizer::{Token, TokenizedInput};
use crate::vocab::VocabDatabase;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The POS tagger itself including the base HMM,
/// along with tag and word based post processing models
#[derive(Default, Serialize, Deserialize)]
pub struct POSTagger {
    pub hmm: HMM<i32>,
    pub cohort: POSModel<i32>,
    pub tags: POSTagModelRepo<i32>,
    pub words: HashMap<i32, POSModel<i32>>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct POSPrediction {
    pub method: POSPredictionMethod,
    pub word: String,
    pub prev_tag: POSTag,
    pub tag: POSTag,
    pub confidence: f32,
    pub probabilities: HashMap<POSTag, f32>,
    pub conjunctions: Vec<String>,
}

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum POSPredictionMethod {
    #[default]
    non_ambiguous,
    hmm,
    standard,
    conjunction,
    deterministic_rule,
    exception,
}

impl POSTagger {
    pub fn new() -> Self {
        Self::default()
    }

    /// Applies part-of-speech tagging to the tokenized input, resolving ambiguous words
    pub fn apply(&self, output: &mut TokenizedInput, vocab: &VocabDatabase) {
        // Fix spelling typos
        self.fix_spelling_typos(output, vocab);

        // Resolve via HMM model
        self.hmm.apply(&mut output.tokens);

        // Iterate through words
        for position in 0..output.tokens.len() {
            if output.tokens[position].potential_pos.len() < 2 {
                continue;
            }

            // Resolve ambiguity
            if let Some(pred) = self.resolve(position, output) {
                output.tokens[position].pos_prediction = pred.clone();
                if output.tokens[position].pos != pred.tag
                    && let Some(new_token) = output.tokens[position].update_pos(pred.tag, vocab)
                {
                    output.tokens[position] = new_token;
                }
            }
        }
    }

    /// Fix spelling typos
    fn fix_spelling_typos(&self, output: &mut TokenizedInput, vocab: &VocabDatabase) {
        for position in 0..output.tokens.len() {
            if output.tokens[position].pos != POSTag::FW {
                continue;
            }

            // Get initial prediction
            if let Some(pred) = self.cohort.predict_cohort(position, &output.tokens) {
                output.tokens[position].pos_prediction = pred;

                // Get spelling correction
                if let Some(correction) =
                    vocab.preprocess.spellchecker.try_correct(position, &output.tokens, vocab)
                {
                    output.tokens[position] = correction;
                }
            }
        }
    }

    // Resolve ambiguity
    fn resolve(&self, position: usize, output: &TokenizedInput) -> Option<POSPrediction> {
        // Check word models
        if let Some(model) = self.words.get(&output.tokens[position].index)
            && let Some(pred) = model.predict(position, &output.tokens)
        {
            return Some(pred);
        }

        // Check tag models
        if let Some(pred) = self.check_tag_models(position, &output.tokens) {
            return Some(pred);
        }

        None
    }

    /// Check the tag models
    fn check_tag_models(&self, position: usize, tokens: &[Token]) -> Option<POSPrediction> {
        let tag = tokens[position].pos;

        // Check tag models
        if let Some(model_names) = self.tags.tags.get(&tag) {
            for name in model_names.iter() {
                let model = self.tags.models.get(&name.to_string()).unwrap();

                // Ensure token is valid for model
                if !model.target_tags.contains(&tag) {
                    continue;
                }
                if !tokens[position]
                    .potential_pos
                    .iter()
                    .filter(|&p_tag| *p_tag != tag)
                    .any(|p_tag| model.target_tags.contains(p_tag))
                {
                    continue;
                }

                if let Some(pred) = model.predict(position, tokens) {
                    return Some(pred);
                }
            }
        }

        None
    }
}

impl POSPrediction {
    pub fn new(
        method: POSPredictionMethod,
        word: &str,
        prev_tag: POSTag,
        tag: POSTag,
        confidence: f32,
        probabilities: &HashMap<POSTag, f32>,
        conjunctions: &[String],
    ) -> Self {
        Self {
            method,
            word: word.to_string(),
            prev_tag,
            tag,
            confidence,
            probabilities: probabilities.clone(),
            conjunctions: conjunctions.to_vec(),
        }
    }
}
