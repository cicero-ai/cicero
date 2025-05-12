// Copyright 2025 Aquila Labs of Alberta, Canada <matt@cicero.sh>
// Licensed under the Functional Source License, Version 1.1 (FSL-1.1)
// See the full license at: https://cicero.sh/license.txt
// Distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND.

use crate::error::Error;
use bincode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// A cache for storing vocabulary-related data, such as typos, to improve processing efficiency.
#[derive(Serialize, Deserialize, Default)]
pub struct VocabCache {
    pub typos: HashMap<String, String>,
}

impl VocabCache {
    /// Loads the vocabulary cache from a file in the specified directory, returning a default cache if the file does not exist.
    pub fn load(vocab_dir: &str) -> Result<VocabCache, Error> {
        let cache_file = format!("{}/cache.dat", vocab_dir);
        if !Path::new(&cache_file).exists() {
            return Ok(Self::default());
        }

        let encoded = fs::read(&cache_file)?;
        let cache: VocabCache = match bincode::deserialize(&encoded[..]) {
            Ok(r) => r,
            Err(e) => {
                return Err(Error::Load(format!(
                    "Unable to load vocabulary cache, {}",
                    e
                )))
            }
        };

        Ok(cache)
    }

    /// Saves the vocabulary cache to a file in the specified directory using bincode serialization.
    pub fn save(&self, vocab_dir: &str) -> Result<(), Error> {
        let cache_file = format!("{}/cache.dat", vocab_dir);
        let encoded = match bincode::serialize(&self) {
            Ok(r) => r,
            Err(e) => {
                return Err(Error::Save(format!(
                    "Unable to serialize vocabulary cache, {}",
                    e
                )))
            }
        };
        fs::write(&cache_file, &encoded)?;
        Ok(())
    }

    /// Adds a typo mapping to the cache, associating the original word with its correct form.
    pub fn add_typo(&mut self, original: &str, correct: &str) {
        self.typos.insert(original.to_string(), correct.to_string());
    }
}
