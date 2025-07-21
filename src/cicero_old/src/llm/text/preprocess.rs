
use std::collections::HashSet;
use regex::Regex;
use serde_derive::{Serialize, Deserialize};
use text_splitter::{Characters, TextSplitter, MarkdownSplitter};
use rust_stemmers::{Algorithm, Stemmer};
use crate::utils::sys;
use crate::server::CONFIG;
use super::PreProcessorConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreProcessor {
    config: PreProcessorConfig
}

impl PreProcessor {

    pub fn new(config: PreProcessorConfig) -> Self {
        Self { config }
    }

    /// Pre-process text
    pub fn process(&self, input: &str) -> Vec<String> { 

        // Initial clean up
        let mut clean_str = self.initial_clean(&input);

        // Stemming
        if self.config.do_stem {
            clean_str = self.stem(&clean_str.as_str());
        }

        // Remove stop words
        if self.config.remove_stop_words {
            clean_str = self.remove_stop_words(&clean_str);
        }

        // Return if not chunking
        if !self.config.do_chunking {
            return vec![clean_str.to_string()];
        }

        // Get chunks
        let chunks = if self.config.is_markdown {
            self.split_chunks_markdown(&clean_str)
        } else {
            self.split_chunks(&clean_str)
        };

        chunks
    }

    /// Initial clean
    fn initial_clean(&self, input: &str) -> String {

        let mut input_lower = if self.config.lowercase {
            input.to_lowercase()
        } else {
            input.to_string()
        };

        let re = Regex::new(r"^[\-\_\=\#\@\!]+").unwrap(); 
        let result = input_lower.split("\n").map(|line| {
            re.replace(line, " ").trim().to_string()
        }).filter(|lc| !lc.is_empty()).collect::<Vec<String>>();

        // Remove non-ascii and other control characters.  Needs research, but may not want to do this for multi-byte charsets.
        let re_non_ascii  = Regex::new(r"[^\x20-\x7E]").unwrap();
        let mut cleaned_str = re_non_ascii.replace_all(&result.join(" ").to_string(), "").to_string();

        // Remove punctuation, if needed
        if self.config.remove_punctuation {
            let punc_regex = Regex::new(r"[[:punct:]]").unwrap();
            cleaned_str = punc_regex.replace_all(&cleaned_str, "").to_string();
        }
        cleaned_str.to_string()
    }

    /// Perform stemming
    pub fn stem(&self, input: &&str) -> String { 

        let stemmer = Stemmer::create(Algorithm::English);
        let words: Vec<String> = input.split(" ").map(|word| stemmer.stem(&word).to_string()).collect(); 

        words.join(" ").to_string()
    }

    /// Remove stop words
    pub fn remove_stop_words(&self, input: &str) -> String { 
        //let stops: HashSet<_> = Spark::stopwords(Language::English).unwrap().iter().collect();
        //let mut words: Vec<&str> = input.split(" ").map(|w| {
            //w.trim()
        //}).filter(|f| f.len() > 1 && !stops.contains(f) && !f.is_empty()).collect();

        input.to_string()
    }

    /// Split text block into chunks for embedding creation
    pub fn split_chunks(&self, input: &str) -> Vec<String> { 

        let max_length = if self.config.chunk_max_length == 0 {
            CONFIG.ml.embedding_chunk_maxlength
        } else {
            self.config.chunk_max_length
        };

        let splitter = TextSplitter::default()
            .with_trim_chunks(true);

        let chunks = splitter.chunks(&input, max_length);
        chunks.map(|c| c.to_string() ).collect::<Vec<String>>()
    }

    /// Split markdown document into chunks for embedding creation
    pub fn split_chunks_markdown(&self, input: &str) -> Vec<String> { 

        let max_length = if self.config.chunk_max_length == 0 {
            CONFIG.ml.embedding_chunk_maxlength
        } else {
            self.config.chunk_max_length
        };

        let splitter = MarkdownSplitter::default()
            .with_trim_chunks(true);

        let chunks = splitter.chunks(&input, max_length);
        chunks.map(|c| c.to_string() ).collect::<Vec<String>>()
    }

    /// Scrub entity name
    pub fn scrub_name(&self, name: &str) -> Vec<String> {

        let re = Regex::new(r"\W").unwrap();
        let words: Vec<String> = name
            .replace("-", " ")
            .to_lowercase()
            .split(" ")
            .map(|w| re.replace_all(&w, "").trim().to_string())
            .filter(|f| !vec!["inc","llc","corp","ltd"].contains(&f.as_str()))
            .collect::<Vec<String>>().clone();;

        words
    }

}


