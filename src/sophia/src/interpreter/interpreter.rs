// Copyright 2025 Aquila Labs of Alberta, Canada <matt@cicero.sh>
// Licensed under the Functional Source License, Version 1.1 (FSL-1.1)
// See the full license at: https://cicero.sh/license.txt
// Distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND.

use super::{CoreferenceCategories, Interpretation, PhraseBuffer};
use crate::interpreter::phrase::{Adjective, Adverb};
use crate::pos_tagger::POSTag;
use crate::tokenizer::{TokenizedInput, Tokenizer};
use crate::vocab::VocabDatabase;
use std::collections::HashMap;

pub struct Interpreter {
    coref_categories: CoreferenceCategories,
}

impl Interpreter {
    /// Creates a new Interpreter instance from the provided vocabulary database.
    pub fn new(vocab: &VocabDatabase) -> Self {
        Self {
            coref_categories: CoreferenceCategories::new(vocab),
        }
    }

    /// Interprets user input by tokenizing, processing, and categorizing tokens into an Interpretation struct.
    /// Returns the constructed Interpretation with scores, tokens, multi-word expressions, and phrases.
    pub fn interpret(
        &self,
        input: &str,
        tokenizer: &Tokenizer,
        vocab: &VocabDatabase,
    ) -> Interpretation {
        // Tokenize input
        let mut tokens = tokenizer.encode(input, vocab);
        let mut buffer = PhraseBuffer::new(&self.coref_categories, vocab);

        // GO through tokens
        for (x, token) in tokens.mwe().enumerate() {
            buffer.tokens.push(token.clone());

            // Check for phrase intent
            if let Some((intent, length)) = vocab.words.phrase_intents.check(x, &tokens) {
                buffer.add_intent(intent, length);
            }

            if token.is_sentence_stopper() {
                buffer.hard_split(x);
            } else if token.is_noun()
                && buffer.last_pos == POSTag::VBG
                && !buffer.current_verbs.is_empty()
            {
                buffer.current_verbs.last_mut().unwrap().objects.push(x);
            } else if token.is_noun() {
                buffer.add_noun(x);
            } else if vocab.preprocess.auxillary_verbs.contains(&token.index)
                || vocab.preprocess.predicative_verbs.contains(&token.index)
            {
                if vocab.preprocess.auxillary_verbs.contains(&token.index) {
                    buffer.auxillary_verbs.push(x);
                }
                if vocab.preprocess.predicative_verbs.contains(&token.index) {
                    buffer.predicative_verbs.push(x);
                }
            } else if token.is_verb() {
                buffer.add_verb(x);
            } else if token.is_adverb() {
                buffer.adverbs.push(Adverb::new(x, &token, vocab));
            } else if token.is_adjective() {
                buffer.adjectives.push(Adjective::new(x, &token, vocab));
            } else if token.is_pronoun() {
                buffer.add_pronoun(x);
            } else if token.is_preposition() {
                buffer.prepositions.push(x);
            } else if token.is_determiner() {
                buffer.determiners.push(x);
            } else if token.pos == POSTag::CC || [",", ";", "-", "+"].contains(&token.word.as_str())
            {
                buffer.noun_seperators.push(x);
            } else if !token.is_conjunction() {
                buffer.noise.push(x);
            }

            // Linker
            if token.is_conjunction() {
                buffer.linkers.push(x);
            }

            // Splitter
            if token.is_preposition() || token.is_conjunction() || token.word.as_str() == "," {
                buffer.splitters.push(x);
            }

            // Add non-pronoun to antecedent buffer
            if !token.is_pronoun() {
                buffer.antecedents.add_non_noun(&token);
            }
            buffer.last_pos = token.pos;
        }

        // Finish buffer
        buffer.hard_split(buffer.tokens.len() - 1);

        // Instantiate interpretation
        Interpretation {
            scores: self.get_scores(&tokens),
            tokens: std::mem::take(&mut tokens.tokens),
            mwe: std::mem::take(&mut buffer.tokens),
            phrases: std::mem::take(&mut buffer.phrases),
        }
    }

    /// Computes classification scores for tokens by averaging scores per code from multi-word expression scoring.
    /// Returns a HashMap mapping classification codes to their average scores.
    fn get_scores(&self, tokens: &TokenizedInput) -> HashMap<i8, f32> {
        let mut res: HashMap<i8, Vec<f32>> = HashMap::new();
        for token in tokens.mwe_scoring() {
            for (code, score) in token.classification_scores.iter() {
                res.entry(*code).or_default().push(score.to_f32());
            }
        }

        // Average scores
        let mut scores: HashMap<i8, f32> = HashMap::new();
        for (code, vec_scores) in res.iter() {
            let avg = vec_scores.iter().sum::<f32>() / (vec_scores.len() as f32);
            scores.insert(*code, avg);
        }

        scores
    }
}
