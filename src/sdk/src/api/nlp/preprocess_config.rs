
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreProcessorConfig {
    pub spellcheck: bool,
    pub lowercase: bool,
    pub remove_stop_words: bool,
    pub remove_punctuation: bool,
    pub do_stem: bool,
    pub is_markdown: bool,
    pub do_chunking: bool,
    pub chunk_max_length: usize
}

impl PreProcessorConfig {

    pub fn builder() -> Self {
        Self::default()
    }

    pub fn spellcheck(mut self, spellcheck: bool) -> Self {
        self.spellcheck = spellcheck;
        self
    }

    pub fn lowercase(mut self, enable: bool) -> Self {
        self.lowercase = enable;
        self
    }

    pub fn remove_stopwords(mut self, remove_stopwords: bool) -> Self {
        self.remove_stop_words = remove_stopwords;
        self
    }

    pub fn remove_punctuation(mut self, remove_punctuation: bool) -> Self {
        self.remove_punctuation = remove_punctuation;
        self
    }

    pub fn do_stem(mut self, stem: bool) -> Self {
        self.do_stem = stem;
        self
    }

    pub fn is_markdown(mut self, is_markdown: bool) -> Self {
        self.is_markdown = is_markdown;
        self
    }

    pub fn do_chunking(mut self, do_chunking: bool) -> Self {
        self.do_chunking = do_chunking;
        self
    }

    pub fn chunk_max_length(mut self, chunk_max_length: usize) -> Self {
        self.chunk_max_length = chunk_max_length;
        self
    }

}

impl Default for PreProcessorConfig {
    fn default() -> PreProcessorConfig {
        PreProcessorConfig {
            spellcheck: true,
            lowercase: true,
            remove_stop_words: false,
            remove_punctuation: false,
            do_stem: false,
            is_markdown: true,
            do_chunking: false,
            chunk_max_length: 0
        }
    }
}


