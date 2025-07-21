// Copyright 2025 Aquila Labs of Alberta, Canada <matt@cicero.sh>
// Licensed under the Functional Source License, Version 1.1 (FSL-1.1)
// See the full license at: https://cicero.sh/license.txt
// Distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND.

pub use self::context::{POSTaggerContext, Feature, FeatureType, DeterministicRule, AnchorIncludes, MAX_TAGS_BEFORE, MAX_TAGS_AFTER, MAX_ANCHORS_BEFORE, MAX_ANCHORS_AFTER};
pub use self::model::{POSModel, POSWordModel, POSLogModel};
pub use self::pos_tag::POSTag;
pub use self::schema::{
    POSTagger, POSTaggerBigramScores, POSTaggerLayer, POSTaggerScores,
    Score,
};

mod context;
mod model;
mod pos_tag;
mod schema;
mod tagger;
