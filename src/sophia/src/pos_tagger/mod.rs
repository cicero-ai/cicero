// Copyright 2025 Aquila Labs of Alberta, Canada <matt@cicero.sh>
// Licensed under the Functional Source License, Version 1.1 (FSL-1.1)
// See the full license at: https://cicero.sh/license.txt
// Distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND.

pub use self::pos_tag::POSTag;
pub use self::schema::{
    POSTagger, POSTaggerBigramScores, POSTaggerLayer, POSTaggerScores,
    Score,
};

mod pos_tag;
mod schema;
mod tagger;
