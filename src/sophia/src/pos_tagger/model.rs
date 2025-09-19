// Copyright 2025 Aquila Labs of Alberta, Canada <matt@cicero.sh>
// Licensed under the PolyForm Noncommercial License 1.0.0
// Commercial use requires a separate license: https://cicero.sh/sophia/
// License text: https://polyformproject.org/licenses/noncommercial/1.0.0/
// Distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND.

use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use serde::{Serialize, Deserialize};
use crate::tokenizer::Token;
use super::{POSTag, POSContext, POSFeature, TokenKey, POSPrediction, POSPredictionMethod, POSSuffix, POSPrefix, SIBLING_TAGS_BEFORE, SIBLING_TAGS_AFTER};

pub trait POSModelInterface {
    fn predict(&self, position: usize, tokens: &[Token]) -> Option<POSPrediction>;
}

#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(bound = "S: Default + Clone + Eq + PartialEq + Hash + Serialize + for<'a> Deserialize<'a>")]
pub struct POSModel<S> {
    pub word: String,
    pub target_tags: Vec<POSTag>,
    pub tag_freq: HashMap<POSTag, f32>,
    pub features: HashMap<POSFeature<S>, POSWeight>,
    pub conjunctions: HashMap<POSFeature<S>, Vec<POSConjunction<S>>>
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "S: Default + Clone + Eq + PartialEq + Hash + Serialize + for<'a> Deserialize<'a>")]
pub struct POSConjunction<S> {
    pub weight: POSWeight,
    pub deterministic_tag: Option<POSTag>,
    pub siblings: Vec<POSFeature<S>>,
    pub exceptions: Vec<(POSFeature<S>, Option<POSTag>)>
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct POSWeight {
    pub tags: HashMap<POSTag, f32>,
    pub weight: f32,
    pub mi_score: f32
}

struct POSPositionTracker<S> {
    primary: Vec<Option<POSScore<S>>>,
    secondary: Vec<HashMap<POSTag, f32>>
}

#[derive(Clone)]
struct POSScore<S> {
    feature: POSFeature<S>,
    tags: HashMap<POSTag, f32>
}

#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(bound = "S: Default + Clone + Eq + PartialEq + Hash + Serialize + for<'a> Deserialize<'a>")]
pub struct POSTagModel<S> {
    pub target_tags: Vec<POSTag>,
    pub global: POSModel<S>,
    pub words: HashMap<S, POSModel<S>>
}

#[derive(Default, Serialize, Deserialize)]
#[serde(bound = "S: Default + Clone + Eq + PartialEq + Hash + Serialize + for<'a> Deserialize<'a>")]
pub struct POSTagModelRepo<S> {
    pub tags: HashMap<POSTag, Vec<String>>,
    pub models: HashMap<String, POSTagModel<S>>, 
}

impl<S> POSModelInterface for POSModel<S>
    where S: Default + Clone + Eq + PartialEq + Hash + Serialize + for<'a> Deserialize<'a>, Token: TokenKey<S>
{

    /// Resolve an ambiguous word
    fn predict(&self, position: usize, tokens: &[Token]) -> Option<POSPrediction> {

        // Get context
        let context = POSContext::from_tokens(position, tokens);
        let context_vec: HashSet<POSFeature<S>> = context.iter_ft().collect();

        // Check conjunctions
        if let Some(pred) = self.check_conjunctions(position, &context, &context_vec, tokens) {
            return Some(pred);
        }

        // Check confidence score

        // Fallback to individual features
        let mut tracker =  POSPositionTracker::new();
        for feature in context.iter_ft() {
            if let Some(weight) = self.features.get(&feature) {
                tracker.add_feature(&feature, weight);
            }
        }

        // Combine scores
        let scores = tracker.combine(&self.tag_freq);
        if scores.is_empty() {
            return None;
        }

        // Get highest score
        let mut scores_vec = scores.iter().collect::<Vec<_>>();
        scores_vec.sort_by(|a,b| b.1.partial_cmp(a.1).unwrap());

        Some(POSPrediction::new(POSPredictionMethod::standard, &tokens[position].word, tokens[position].pos, *scores_vec[0].0, *scores_vec[0].1, &scores, &[]))
    }
}

impl<S> POSModel<S> 
    where S: Default + Clone + Eq + PartialEq + Hash + Serialize + for<'a> Deserialize<'a>, Token: TokenKey<S>
{

    /// Check conjunctions
    fn check_conjunctions(&self, position: usize, context: &POSContext<S>, context_vec: &HashSet<POSFeature<S>>, tokens: &[Token]) -> Option<POSPrediction> {
        let mut scores: HashMap<POSTag, f32> = HashMap::new();

        // Go through context
        for feature in context_vec.iter() {

            let conjunction_set = match self.conjunctions.get(feature) {
                Some(r) => r,
                None => continue
            };

            // Find strongest matching conjunction, if any
            for conjunction in conjunction_set.iter() {

                // Check exceptions
                if let Some(tag) = self.check_exceptions(conjunction, context_vec) {
                if tag.is_none() { continue; }
                return Some(POSPrediction::new(POSPredictionMethod::exception, &tokens[position].word, tokens[position].pos, tag.unwrap(), 1.0, &HashMap::new(), &[]));
            }

                // Check siblings
                if !conjunction.siblings.iter().all(|sib| {
                    let offset = ((feature.offset + sib.offset) + (SIBLING_TAGS_BEFORE as i8)) as usize;
                    context.0[offset].contains(&sib.feature_token)
                }) { 
                    continue;
                }

                // Check for deterministic tag
                if let Some(tag) = conjunction.deterministic_tag {
                    return Some(POSPrediction::new(POSPredictionMethod::deterministic_rule, &tokens[position].word, tokens[position].pos, tag, 1.0, &HashMap::new(), &[]));
                }

                // Add to results
                for (tag, score) in conjunction.weight.tags.iter() {
                    *scores.entry(*tag).or_insert(0.0) += *score * conjunction.weight.weight;
                }
                break;
            }
        }

        // Check for none
        if scores.is_empty() {
            return None;
        }

        // Get highest score
        let mut scores_vec = scores.iter().collect::<Vec<_>>();
        scores_vec.sort_by(|a,b| b.1.partial_cmp(a.1).unwrap());

        Some(POSPrediction::new(POSPredictionMethod::conjunction, &tokens[position].word, tokens[position].pos, *scores_vec[0].0, *scores_vec[0].1, &scores, &[]))
    }

    /// Check exceptions
    fn check_exceptions(&self, conjunction: &POSConjunction<S>, context_vec: &HashSet<POSFeature<S>>) -> Option<Option<POSTag>> {

        for (exception, opt_tag) in conjunction.exceptions.iter() {
            if !context_vec.contains(exception) { continue; }

            if let Some(_tag) = opt_tag {
                //return Some(Some(*tag));
                return Some(None);
            } else {
                return Some(None);
            }
        }

        None
    }

    /// Predict cohort, used for automated spelling corrections to narraow search space of candidates
    pub fn predict_cohort(&self, position: usize, tokens: &[Token]) -> Option<POSPrediction> {

        // Get initial prediction
        let mut pred = self.predict(position, tokens)?;
        let max_value = pred.probabilities.values().sum::<f32>();

        // Scale and normalize probabilities
        for (tag, score) in pred.probabilities.iter_mut() {
            let overall_score = self.tag_freq.get(tag).unwrap_or(&0.0);
            *score /= max_value;
            *score *= (0.10 / overall_score.max(1e-6)).sqrt();
            //*score *= (0.10 / *overall_score);
        }

        // Add suffix bonus
        if let Ok(suffix) = POSSuffix::try_from(&tokens[position]) {
            let (suffix_tag, suffix_bonus) = match suffix {
                POSSuffix::ed|POSSuffix::ing => (POSTag::VB, 0.10),
                POSSuffix::day|POSSuffix::ion|POSSuffix::tion|POSSuffix::ness|POSSuffix::ment|POSSuffix::ity|POSSuffix::ty|POSSuffix::ance|POSSuffix::ence|POSSuffix::age|POSSuffix::ship|POSSuffix::hood => (POSTag::NN, 0.15),
                POSSuffix::wise => (POSTag::RB, 0.15),
                POSSuffix::ly|POSSuffix::ward => (POSTag::RB, 0.15),
                POSSuffix::er|POSSuffix::est|POSSuffix::ous|POSSuffix::less|POSSuffix::ful|POSSuffix::able|POSSuffix::ible => (POSTag::JJ, 0.15),
                POSSuffix::al|POSSuffix::ive => (POSTag::JJ, 0.10),
                _ => (POSTag::FW, 0.0)
            };

            if pred.probabilities.contains_key(&suffix_tag) {
                *pred.probabilities.get_mut(&suffix_tag).unwrap() += suffix_bonus;
            }
        }

        // Add prefix bonus
        if let Ok(prefix) = POSPrefix::try_from(&tokens[position]) {
            let(prefix_tag, prefix_bonus) = match prefix {
                POSPrefix::non|POSPrefix::anti|POSPrefix::semi|POSPrefix::uni|POSPrefix::bi|POSPrefix::tri|POSPrefix::quad|POSPrefix::mono|POSPrefix::pseudo|POSPrefix::quasi => (POSTag::JJ, 0.075),
                POSPrefix::en|POSPrefix::em|POSPrefix::mis => (POSTag::VB, 0.075),
                POSPrefix::sub|POSPrefix::inter|POSPrefix::intra|POSPrefix::trans => (POSTag::NN, 0.075),
                POSPrefix::un|POSPrefix::pre|POSPrefix::over|POSPrefix::micro|POSPrefix::mega|POSPrefix::extra|POSPrefix::poly => (POSTag::JJ, 0.05),
                POSPrefix::re|POSPrefix::dis|POSPrefix::de => (POSTag::VB, 0.05),
                POSPrefix::co|POSPrefix::com|POSPrefix::post|POSPrefix::fore => (POSTag::NN, 0.05),
                _ => (POSTag::FW, 0.0)
            };

            if pred.probabilities.contains_key(&prefix_tag) {
                *pred.probabilities.get_mut(&prefix_tag).unwrap() += prefix_bonus;
            }
        }

        // Normalize
        let total = pred.probabilities.values().sum::<f32>();
        for (_, score) in pred.probabilities.iter_mut() {
            *score /= total;
        }

        // GEt highest ranking probability
        let mut scores_vec = pred.probabilities.iter().collect::<Vec<_>>();
        scores_vec.sort_by(|a,b| b.1.partial_cmp(a.1).unwrap());

        // Set new tag
        pred.tag = *scores_vec[0].0;
        pred.confidence = *scores_vec[0].1;

        Some(pred)
    }

}


impl<S> POSModelInterface for POSTagModel<S> 
    where S: Default + Clone + Eq + PartialEq + Hash + Serialize + for<'a> Deserialize<'a>, Token: TokenKey<S>
{
/// Predict tag for an ambiguous word
    fn predict(&self, position: usize, tokens: &[Token]) -> Option<POSPrediction> {

        // Check per-word models
        if let Some(model) = self.words.get(&tokens[position].get_key())
            && let Some(pred) = model.predict(position, tokens)
                && pred.confidence >= 0.85 {
                    return Some(pred);
                }

        self.global.predict(position, tokens)
    }
}

impl<S> POSPositionTracker<S>
    where S: Default + Clone + Eq + PartialEq + Hash + Serialize + for<'a> Deserialize<'a>, Token: TokenKey<S>
{
    pub fn new() -> Self {
        let length = SIBLING_TAGS_BEFORE + SIBLING_TAGS_AFTER + 1;

        Self {
            primary: vec![None; length],
            secondary: vec![HashMap::new(); length]
        }
    }

    // Update with a new features
    pub fn add_feature(&mut self, feature: &POSFeature<S>, weight: &POSWeight) {
        let index = (SIBLING_TAGS_BEFORE as i8 + feature.offset) as usize;

        // Secondary feature
        if !feature.feature_token.is_primary() {
            for (tag, score) in weight.tags.iter() {
                *self.secondary[index].entry(*tag).or_insert(0.0) += weight.weight * *score;
            }

        // Primary feature
        } else if self.primary[index].is_none() || feature.get_score() > self.primary[index].as_ref().unwrap().feature.get_score() {
            let tags: HashMap<POSTag, f32> = weight.tags.iter().map(|(tag, score)| {
                (*tag, (weight.weight * *score))
            }).collect();

            self.primary[index] = Some(POSScore {
                feature: feature.clone(),
                tags
            });
        }

    }

    /// Combine scores
    pub fn combine(&self, tag_freq: &HashMap<POSTag, f32>) -> HashMap<POSTag, f32> {

        let mut scores: HashMap<POSTag, f32> = HashMap::new();
        for (x, score_opt) in self.primary.iter().enumerate() {

            let score = match score_opt {
                Some(r) => r,
                None => continue
            };

            for (tag, tag_score) in score.tags.iter() {
                *scores.entry(*tag).or_insert(0.0) += *tag_score;
            }

            // Add secondary
            for (tag, tag_score) in self.secondary[x].iter() {
            *scores.entry(*tag).or_insert(0.0) += *tag_score;
            }
        }

        // Add tag freq
        for (tag, score) in scores.iter_mut() {
            if let Some(freq_score) = tag_freq.get(tag) {
                *score = (*score * 0.8) + (freq_score * 0.2);
            }
        }

        scores
    }
}

