
use self::nlp::PreProcessorConfig;

pub mod nlp;

pub trait Cicero {
    fn std(&self) -> &Box<dyn CiceroSTD>;
    fn nlp(&self) -> &Box<dyn CiceroNLP>;
}

pub trait CiceroSTD {
    fn encrypt(&self, message: &[u8], password: &[u8; 32]) -> Vec<u8>;
    fn decrypt(&self, payload: &[u8], password: [u8; 32]) -> Option<Vec<u8>>;
}

pub trait CiceroNLP {
    fn set_preprocessor_config(&mut self, config: &PreProcessorConfig);
    fn preprocess_text(&self, input: &str) -> Vec<String>;
}


