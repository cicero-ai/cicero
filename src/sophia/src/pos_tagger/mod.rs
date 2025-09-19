// Copyright 2025 Aquila Labs of Alberta, Canada <matt@cicero.sh>
// Licensed under the PolyForm Noncommercial License 1.0.0
// Commercial use requires a separate license: https://cicero.sh/sophia/
// License text: https://polyformproject.org/licenses/noncommercial/1.0.0/
// Distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND.

pub use self::context::{
    AUXILLARY_VERBS, COMMON_ADVERBS, PASSIVE_INDICATORS, POSContext, POSFeature, POSFeatureToken,
    POSPrefix, POSSuffix, POSTagGroup, POSWordGroup, SIBLING_TAGS_AFTER, SIBLING_TAGS_BEFORE,
    TEMPORAL_ADVERBS,
};
pub use self::hmm::{HMM, TOTAL_TAGS};
pub use self::model::{
    POSConjunction, POSModel, POSModelInterface, POSTagModel, POSTagModelRepo, POSWeight,
};
pub use self::pos_tag::POSTag;
pub use self::tagger::{POSPrediction, POSPredictionMethod, POSTagger};
use crate::tokenizer::Token;

mod context;
mod hmm;
mod model;
mod pos_tag;
mod tagger;

pub trait TokenKey<S> {
    fn get_key(&self) -> S;
}

impl TokenKey<i32> for Token {
    fn get_key(&self) -> i32 {
        self.index
    }
}

impl TokenKey<String> for Token {
    fn get_key(&self) -> String {
        self.word.to_string()
    }
}
