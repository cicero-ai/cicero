use super::{
    FutureVerbPhrases, PhraseIntents, SpellChecker, VocabCache, VocabCategoryDatabase, VocabMWE,
};
use crate::error::Error;
use crate::pos_tagger::{POSTag, POSTagger};
use crate::tokenizer::Token;
use crate::vocab::f8::f8;
use crate::vocab::mwe::Capitalization;
use bincode;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Mutex;

/// A comprehensive vocabulary database for natural language processing, containing metadata, preprocessing data, words, categories, and a cache.
#[derive(Serialize, Deserialize)]
pub struct VocabDatabase {
    pub meta: VocabDatabaseMeta,
    pub preprocess: VocabPreProcessDatabase,
    pub words: VocabWordDatabase,
    pub categories: VocabCategoryDatabase,
    #[serde(skip_serializing, skip_deserializing)]
    pub cache: Mutex<VocabCache>,
}

/// Metadata for the vocabulary database, including version, language, author, and integrity details.
#[derive(Serialize, Deserialize)]
pub struct VocabDatabaseMeta {
    version: (i8, i8, i8),
    language: String,
    author: String,
    creation_time: String,
    sha256_hash: String,
    signature: String,
    comment: String,
}

/// Preprocessing data for the vocabulary, including hashes, typos, spellchecker, verb prefixes, and other linguistic resources.
#[derive(Serialize, Deserialize, Clone)]
pub struct VocabPreProcessDatabase {
    pub hashes: HashMap<String, (String, String)>,
    pub spellchecker: SpellChecker,
    pub future_verb_prefixes: Vec<String>,
    pub stop_words: Vec<i32>,
    pub predicative_verbs: Vec<i32>,
    pub auxillary_verbs: Vec<i32>,
    pub infinitive_prefixes: Vec<i32>,
}

/// Word-specific data for the vocabulary, including word lists, POS tagger, MWEs, capitalization, future verbs, and token mappings.
#[derive(Serialize, Deserialize)]
pub struct VocabWordDatabase {
    pub wordlist: HashMap<String, IndexMap<POSTag, i32>>,
    pub pos_tagger: POSTagger<f8, i32>,
    pub mwe: VocabMWE,
    pub capitalization: HashMap<i32, Capitalization>,
    pub future_verbs: FutureVerbPhrases,
    pub phrase_intents: PhraseIntents,
    pub id2token: HashMap<i32, Token>,
    pub plural: HashMap<i32, i32>,
}

impl VocabDatabase {
    /// Saves the vocabulary database to a file using bincode serialization.
    pub fn save(&mut self, filename: &str) -> Result<(), Error> {
        let encoded: Vec<u8> = match bincode::serialize(&self) {
            Ok(r) => r,
            Err(e) => {
                return Err(Error::Save(format!(
                    "Unable to serialize vocabulary data store, {}",
                    e
                )))
            }
        };
        fs::write(filename, &encoded)?;
        Ok(())
    }

    /// Loads a vocabulary database from a file in the specified directory, initializing the cache.
    pub fn load(datadir: &str, language: &str) -> Result<VocabDatabase, Error> {
        let filename = format!("{}/{}.dat", datadir, language);
        if !Path::new(&filename).exists() {
            return Err(Error::Load(format!(
                "No vocabulary file exists at, {}",
                filename
            )));
        }
        let contents = fs::read(&filename)?;

        let mut vocab: VocabDatabase = match bincode::deserialize(&contents[..])
        {
            Ok(r) => r,
            Err(e) => return Err(Error::Load(format!("Unable to load the vocabulary file.  Please ensure correct file is in place, and re-download from secure client area if necessary.  Contact customer support if the problem persists.  Error: {}", e)))
        };

        vocab.cache = Mutex::new(VocabCache::load(datadir)?);
        Ok(vocab)
    }

    /// Looks up a word by string, returning a Token based on its vocabulary entry.
    pub fn from_str(&self, word: &str) -> Token {
        let (_, lookup) = match self.lookup_word(word) {
            Some(r) => r,
            None => return Token::default(),
        };

        // get token
        let token_id = lookup.values().next().unwrap();
        self.words.id2token.get(&token_id.clone()).unwrap().clone()
    }

    /// Converts a word to its corresponding token ID.
    pub fn to_int(&self, word: &str) -> i32 {
        let token = self.from_str(word);
        token.index
    }

    /// Looks up a word in the vocabulary, returning its string and POS-to-ID mapping if found.
    pub fn lookup_word(&self, word: &str) -> Option<(String, IndexMap<POSTag, i32>)> {
        // Check mwe
        if word.contains(" ") {
            return None;
        }

        // Straight lookup
        if let Some(index) = self.words.wordlist.get(&word.to_string()) {
            return Some((word.to_string(), index.clone()));
        }

        // Lowercase lookup
        if let Some(index) = self.words.wordlist.get(&word.to_lowercase()) {
            return Some((word.to_string(), index.clone()));
        }

        None
    }

    /// Creates a Token from a given token ID using the vocabulary database.
    pub fn from_int(&self, token_id: &i32) -> Token {
        Token::from_id(*token_id, self)
    }
}

impl Default for VocabDatabaseMeta {
    fn default() -> VocabDatabaseMeta {
        VocabDatabaseMeta {
            version: (1, 0, 0),
            language: "en".to_string(),
            author: "cicero".to_string(),
            creation_time: String::new(),
            sha256_hash: String::new(),
            signature: String::new(),
            comment: String::new(),
        }
    }
}
