

use cicero_sdk::api::CiceroNLP;
use crate::llm::text::{PreProcessor, PreProcessorConfig};

#[derive(Debug, Clone)]
pub struct CiceroAPI_NLP {
    preprocessor: PreProcessor
}

impl CiceroAPI_NLP {

    pub fn new() -> Self {
        Self {
            preprocessor: PreProcessor::new(Default::default())
        }
    }

}


impl CiceroNLP for CiceroAPI_NLP {

    /// Set pre-processor config
    fn set_preprocessor_config(&mut self, config: &PreProcessorConfig) {
        self.preprocessor = PreProcessor::new(config.clone());
    }

    /// Pre-process and clean input text
    fn preprocess_text(&self, input: &str) -> Vec<String> {
        self.preprocessor.process(&input)
    }

}


