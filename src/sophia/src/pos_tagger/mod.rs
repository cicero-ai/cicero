// Copyright 2025 Aquila Labs of Alberta, Canada <matt@cicero.sh>
// Licensed under the PolyForm Noncommercial License 1.0.0
// Commercial use requires a separate license: https://cicero.sh/sophia/
// License text: https://polyformproject.org/licenses/noncommercial/1.0.0/
// Distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND.

use crate::tokenizer::Token;
pub use self::context::{POSContext, POSFeature, POSFeatureToken, POSTagGroup, POSWordGroup, POSSuffix, POSPrefix, SIBLING_TAGS_BEFORE, SIBLING_TAGS_AFTER, AUXILLARY_VERBS, PASSIVE_INDICATORS, COMMON_ADVERBS, TEMPORAL_ADVERBS};
pub use self::hmm::{HMM, TOTAL_TAGS};
pub use self::model::{POSModel, POSModelInterface, POSConjunction, POSWeight, POSTagModel, POSTagModelRepo};
pub use self::pos_tag::POSTag;
pub use self::tagger::{POSTagger, POSPrediction, POSPredictionMethod};

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


