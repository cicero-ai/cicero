// Copyright 2025 Aquila Labs of Alberta, Canada <matt@cicero.sh>
// Licensed under the Functional Source License, Version 1.1 (FSL-1.1)
// See the full license at: https://cicero.sh/license.txt
// Distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND.

use crate::error::Error;
use crate::interpreter::{Interpretation, Interpreter};
use crate::tokenizer::{Token, TokenizedInput, Tokenizer};
use crate::vocab::{VocabCategory, VocabDatabase, VocabStats};

/// The main entry point for the Sophia natural language processing library, integrating tokenization and interpretation capabilities.
///
/// The main entry point into the Sophia NLU engine, and contains everything you need for NLU (natural languaging understanding) tasks.  
/// Allows simple tokenization of input into both, individual words and MWEs (multi-word entities) mixed with individual words, along with
///  interpreting  user input and breaking it down into usable phrase, noun, verb and other constructs.
pub struct Sophia {
    pub datadir: String,
    _language: String,
    pub vocab: VocabDatabase,
    pub tokenizer: Tokenizer,
    pub interpreter: Interpreter,
}

impl Sophia {
    /// Creates a new `Sophia` instance, loading the vocabulary database from the specified directory and language.
    ///
    /// # Arguments
    /// - `datadir`: The path to the directory containing the vocabulary database files.
    /// - `language`: The language code and filename of the .dat vocabulary file (eg. 'en' for 'en.dat' file)
    ///
    /// # Returns
    /// A `Result` containing the initialized `Sophia` instance or an `Error` if the vocabulary cannot be loaded.
    ///
    pub fn new(datadir: &str, language: &str) -> Result<Self, Error> {
        let vocab = VocabDatabase::load(datadir, language)?;

        Ok(Self {
            datadir: datadir.to_string(),
            _language: language.to_string(),
            interpreter: Interpreter::new(&vocab),
            tokenizer: Tokenizer::new(),
            vocab,
        })
    }

    /// Tokenizes the input text into a `TokenizedInput` containing tokens and MWEs.
    ///
    /// This method processes the input string using the `Tokenizer`, breaking it into individual tokens and identifying multi-word entities (MWEs).
    /// The resulting `TokenizedInput` can be iterated to access tokens or MWEs, with optional filtering for stopwords.
    ///
    /// # Arguments
    /// - `input`: The text to tokenize.
    ///
    /// # Returns
    /// A `Result` containing a `TokenizedInput` with the tokenized representation of the input text, or an `Error` if tokenization fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// #![no_run]
    /// use sophia::{Sophia, Error};
    ///
    /// fn main() -> Result<(), Error> {
    ///     let sophia = Sophia::new("./vocab_data", "en")?;
    ///     let output = sophia.tokenize("The quick brown fox jumps")?;
    ///
    ///     // Iterate over individual tokens
    ///     for token in output.iter() {
    ///         println!("Word: {}, POS: {}", token.word, token.pos);
    ///     }
    ///
    ///     // Iterate over MWEs
    ///     for token in output.mwe() {
    ///         println!("MWE: {}, POS: {}", token.word, token.pos);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn tokenize(&self, input: &str) -> TokenizedInput {
        self.tokenizer.encode(input, &self.vocab)
    }

    /// Interprets the input text, and returns an `Interpretation` with tokens, MWEs and usable phrases.
    ///
    /// This method first tokenizes the input using the `Tokenizer` and then processes the tokens using the `Interpreter` to generate a structured
    /// interpretation. The result includes individual tokens, MWEs, and phrases with associated scores for semantic analysis.
    ///
    /// # Arguments
    /// - `input`: The text to interpret.
    ///
    /// # Returns
    /// A `Result` containing an `Interpretation` with the analyzed structure of the input text, or an `Error` if tokenization or interpretation fails.
    /// # Example
    ///
    /// ```rust
    /// #![no_run]
    /// use sophia::{Sophia, Error};
    ///
    /// fn main() -> Result<(), Error> {
    ///     let sophia = Sophia::new("./vocab_data", "en")?;
    ///     let output = sophia.interpret("The quick brown fox jumps over the fallen tree while running through the forest with his friends.")?;
    ///
    ///     // Iterate over phrases
    ///     for phrase in output.phrases.iter() {
    ///         println!("Phrase: {:?}", phrase);
    ///         for noun in phrase.nouns.iter() {
    ///             println!("Noun Head: {}", output.tokens[noun.head].word);
    ///         }
    ///
    ///         for verb in phrase.verbs.iter() {
    ///             println!("Verb Head: {}", output.tokens[verb.head].word);
    ///         }
    ///
    ///     // Iterate over individual tokens
    ///     for token in output.tokens.iter() {
    ///         println!("Word: {}, POS: {}", token.word, token.pos);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn interpret(&self, input: &str) -> Interpretation {
        self.interpreter.interpret(input, &self.tokenizer, &self.vocab)
    }

    /// Gets an individual token by its index id#
    ///
    /// # Arguments
    /// - `index`: The index id# of the token to retrieve
    ///
    /// # Returns
    /// A `Option` containing the `Token` or None if the index id# does not exist.
    /// # Example
    ///
    /// ```rust
    /// #![no_run]
    /// use sophia::{Sophia, Error};
    ///
    /// fn main() -> Result<(), Error> {
    ///     let sophia = Sophia::new("./vocab_data", "en")?;
    ///     let output = sophia.tokenize("She was running down the road");
    ///
    ///     // Get the stem of 'running'
    ///     let index = output.tokens[2].stem;   // the index id# of the stem of 'running'.
    ///     if let Some(token) = sophia.get_token(index) {
    ///         println!("Stem of running is {}", token.word);
    ///     }
    ///     Ok(())
    /// }
    pub fn get_token(&self, index: i32) -> Option<Token> {
        let token = self.vocab.words.id2token.get(&index)?;
        let mut res = token.clone();
        res.index = index;
        Some(res)
    }

    /// Gets an individual token by word.
    ///
    /// # Arguments
    /// - `word`: The word to lookup and retrieve `Token` for.
    ///
    /// # Returns
    /// A `Option` containing the `Token` or None if the word does not exist.
    /// # Example
    ///
    /// ```rust
    /// #![no_run]
    /// use sophia::{Sophia, Error};
    ///
    /// fn main() -> Result<(), Error> {
    ///     let sophia = Sophia::new("./vocab_data", "en")?;
    ///     if let Some(token) = sophia.get_word("running") {
    ///         println!("got token {}", token);
    ///     }
    ///     Ok(())
    /// }
    pub fn get_word(&self, word: &str) -> Option<Token> {
        // Check wordlist
        let pos_map = self.vocab.words.wordlist.get(word)?;

        // Get token
        let (_, index) = pos_map.first().unwrap();
        let token = self.vocab.words.id2token.get(index)?;
        let mut res = token.clone();

        res.index = *index;
        res.potential_pos = pos_map.keys().map(|tag| *tag).collect();

        Some(res)
    }

    /// Gets a category by its path.
    ///
    /// # Arguments
    /// - `category_path`: The full category path to lookup (eg. verbs/action/search/retrieve/pursue)
    ///
    /// # Returns
    /// A `Option` containing the `VocabCategory` or None if the word does not exist.
    /// # Example
    ///
    /// ```rust
    /// #![no_run]
    /// use sophia::{Sophia, Error};
    ///
    /// fn main() -> Result<(), Error> {
    ///     let sophia = Sophia::new("./vocab_data", "en")?;
    ///     if let Some(cat) = sophia.get_category("verbs/action/search/retrieve/pursue") {
    ///         println!("got category fqn {}", cat.fqn);
    ///     }
    ///     Ok(())
    /// }
    pub fn get_category(&self, category_path: &str) -> Option<VocabCategory> {
        self.vocab.categories.get_category_by_path(category_path, &self.vocab)
    }

    /// Returns various statistics regarding the loaded vocabulary file such as total singular / ambiguous words, MWEs, POS tags, and more.
    pub fn get_vocab_stats(&self) -> VocabStats {
        VocabStats::compile(&self.vocab)
    }
}
