// Copyright 2025 Aquila Labs of Alberta, Canada <matt@cicero.sh>
// Licensed under the Functional Source License, Version 1.1 (FSL-1.1)
// See the full license at: https://cicero.sh/license.txt
// Distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND.

use std::hash::Hash;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use smartcore::linear::logistic_regression::LogisticRegression;
use smartcore::linalg::basic::matrix::DenseMatrix;
use super::{POSTag, Feature, DeterministicRule};

pub type POSLogModel = LogisticRegression<f32, u8, DenseMatrix<f32>, Vec<u8>>;

#[derive(Default, Serialize, Deserialize)]
#[serde(
    bound = "S: Default + Eq + PartialEq + Hash + Serialize + for<'a> Deserialize<'a>"
)]
pub struct POSModel<S> {
    pub tags: Vec<POSTag>,
    pub words: HashMap<S, POSWordModel>
}

#[derive(Serialize, Deserialize)]
pub struct POSWordModel {
    pub deterministic_rules: Vec<DeterministicRule>,
    pub features: Vec<Feature>,
    pub catchall: Option<POSTag>,
    pub model: Option<POSLogModel>,
    pub model_score: (f32, u32),
    pub exceptions: Vec<u8>
}

impl<S: Default + Hash + Eq + Serialize + for<'de> Deserialize<'de>> POSModel<S> {
    pub fn new() -> Self {
        Self::default()
    }

}


