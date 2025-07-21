
use std::collections::HashMap;
use serde_derive::{Serialize, Deserialize};
use super::{PreProcessor, PreProcessorConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TF_IDF {
    pre_processor: PreProcessor,
    pub words: HashMap<String, usize>
}

impl TF_IDF {

    pub fn new() -> Self {
        Self::default()
    }

    /// Add text block
    pub fn add(&mut self, input: &str) {

        // Pre-processor
        let clean_input = self.pre_processor.process(&input);
        let words: Vec<String> = clean_input[0].split(" ").map(|w| w.to_string()).collect();

        // Go through all words
        for word in words {
            *self.words.entry(word.to_string()).or_insert(0) += 1;
        }

    }

}

impl Default for TF_IDF {
    fn default() -> TF_IDF {
        let mut config = PreProcessorConfig::builder().do_stem(true).remove_punctuation(true);
        Self { 
            pre_processor: PreProcessor::new(config),
            words: HashMap::new()
        }
    }

}

