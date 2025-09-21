// Copyright 2025 Aquila Labs of Alberta, Canada <matt@cicero.sh>
// Licensed under the PolyForm Noncommercial License 1.0.0
// Commercial use requires a separate license: https://cicero.sh/sophia/
// License text: https://polyformproject.org/licenses/noncommercial/1.0.0/
// Distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND.

use super::{MWE, Token, TokenCleaner, TokenizedInput};
use crate::pos_tagger::POSTag;
use crate::vocab::{MWEType, VocabDatabase};
use regex::Regex;

static PREFIX_PAST_DATE_PERIOD_WORDS: &[&str] = &["last", "past", "prior"];
static PREFIX_FUTURE_DATE_PERIOD_WORDS: &[&str] = &["in", "next", "within", "following"];
static SUFFIX_PAST_DATE_PERIOD_WORDS: &[&str] = &["ago", "prior", "earlier", "before", "since"];
static SUFFIX_FUTURE_DATE_PERIOD_WORDS: &[&str] = &["later", "ahead", "afterwards"];
static PREFIX_PAST_DATE_WORDS: &[&str] = &["last", "past", "prior"];
static PREFIX_FUTURE_DATE_WORDS: &[&str] = &["next", "following"];

/// A tokenizer for converting input text into tokens, handling multi-word entities (MWEs) and special cases like contractions and dates.
#[derive(Default)]
pub struct Tokenizer {}

/// A buffer for tokenization, storing output tokens, words, and state for handling MWEs, possessives, and special tags.
#[derive(Default)]
pub struct Buffer {
    pub output: TokenizedInput,
    pub words: Vec<String>,
    pub prev_tag: String,
    pub is_possessive: bool,
    pub not_position: Option<usize>,
    pub have_position: Option<usize>,
    pub had_position: Option<usize>,
    pub suffix: Vec<Token>,
    pub mwe_length: usize,
    pub mwe_scoring_length: usize,
}

impl Tokenizer {
    /// Creates a new Tokenizer instance.
    pub fn new() -> Self {
        Self {}
    }

    /// Encodes input text into a TokenizedInput, processing words, MWEs, and special tags using the vocabulary database.
    pub fn encode(&self, input: &str, vocab: &VocabDatabase) -> TokenizedInput {
        // Clean str
        let clean_str = self.initial_clean(input);
        let mut buffer = Buffer::new(input, &clean_str);

        // Go through words
        while !buffer.words.is_empty() {
            // Get next word
            let mut word = buffer.words.remove(0);
            if buffer.words.is_empty() {
                continue;
            }

            // Initial check
            if buffer.prev_tag.as_str() == "|num|"
                && vocab.preprocess.hashes.contains_key(&word.to_lowercase())
            {
                let (tag, unit) = vocab.preprocess.hashes.get(&word.to_lowercase()).unwrap();
                buffer.push_token(Token::special(&word, tag, "", unit, vocab));
                continue;
            }
            let mut token = Token::new(&word, vocab);

            // Pre-process -- punctuation, special placeholders, etc.
            if token.pos == POSTag::FW {
                word = self.preprocess(word, vocab, &mut buffer);
                if word.is_empty() {
                    continue;
                }
                token = Token::new(&word, vocab);
            }

            // Check mwe
            self.check_mwe(&word, &token, vocab, &mut buffer);

            // Add token
            buffer.push_token(token);
            buffer.push_suffix();
        }

        if !buffer.suffix.is_empty() {
            //buffer.output.tokens.extend_from_slice(&buffer.suffix);
            buffer.push_suffix();
            buffer.suffix.clear();
        }
        //println!("Before word {} index {} tag {}", buffer.output.tokens[47].word, buffer.output.tokens[47].index, buffer.output.tokens[47].pos.to_string());
        // Apply POS tagging
        vocab.words.pos_tagger.apply(&mut buffer.output, vocab);

        buffer.output
    }

    /// Performs initial cleaning of input text, removing non-ASCII characters, leading symbols, and adding newline markers.
    fn initial_clean(&self, input: &str) -> String {
        let re = Regex::new(r"^[\-\_\=\#\@\!]+").unwrap();
        let result = input
            .split("\n")
            .map(|line| format!("{} |NL| ", re.replace(line, " ").trim()))
            .filter(|lc| !lc.is_empty())
            .collect::<Vec<String>>();

        // Remove non-ascii asnd other control characters.  (needs removal / refinement upon support for multi-type char sets)
        let re_non_ascii = Regex::new(r"[^\x20-\x7E]").unwrap();
        let cleaned_str = re_non_ascii.replace_all(&result.join(" "), "").to_string();

        cleaned_str.trim().to_string()
    }

    /// Pre-processes a word, cleaning it and handling special cases like contractions, numbers, and dates, updating the buffer.
    fn preprocess(&self, mut word: String, vocab: &VocabDatabase, buffer: &mut Buffer) -> String {
        // Clean token
        let mut cleaner = TokenCleaner::new();
        word = match cleaner.clean(word, vocab, buffer) {
            Some(r) => r,
            None => return String::new(),
        };
        // Check pre-processor hashes
        let (tag, value) = match vocab.preprocess.hashes.get(&word.to_lowercase()) {
            Some(r) => r,
            None => return word.to_string(),
        };
        // Contraction
        if tag.as_str() == "|contraction|" {
            let words: Vec<String> = value.split(" ").map(String::from).collect();
            for tmp_word in words.iter().skip(1).rev() {
                buffer.words.insert(0, tmp_word.clone());
            }
            return words[0].clone();
        } else if buffer.prev_tag.as_str() == "|num|"
            || ["|day_of_week|", "|month|"].contains(&tag.as_str())
        {
            buffer.push_token(Token::special(&word, tag, "", value, vocab));
            return String::new();
        } else if tag.as_str() == "|num|" {
            buffer.push_token(Token::special(&word, tag, value, "", vocab));
            return String::new();
        }

        word
    }

    /// Checks for multi-word entities (MWEs) and future verb phrases, updating the buffer if found.
    fn check_mwe(&self, word: &String, token: &Token, vocab: &VocabDatabase, buffer: &mut Buffer) {
        // Only process if both buffers are empty
        if buffer.words.is_empty() || (buffer.mwe_length > 0 && buffer.mwe_scoring_length > 0) {
            return;
        }

        // Check for a future verb
        if buffer.mwe_length == 0
            && vocab.preprocess.future_verb_prefixes.contains(&word.to_lowercase())
            && self.check_future_verb(word, vocab, buffer)
        {
            return;
        }

        // Get first index
        let mut index = match vocab.words.mwe.children.get(&word.to_lowercase()) {
            Some(r) => r,
            None => return,
        };

        // Start MWEs
        let mut mwe = vec![index.format(word)];
        let mut mwe_scoring = mwe.clone();
        let (mut mwe_index, mut mwe_scoring_index) = (0, 0);

        // Loop until we can't anymore
        let mut x = 0;
        while let Some(next) = index.children.get(&buffer.words[x].to_lowercase().to_string()) {
            if next.mwe_type == MWEType::standard || next.mwe_type == MWEType::both {
                mwe.push(next.format(&buffer.words[x]));
                if next.index > 0 {
                    mwe_index = next.index;
                }
            }

            if next.mwe_type == MWEType::scoring || next.mwe_type == MWEType::both {
                mwe_scoring.push(next.format(&buffer.words[x]));
                if next.index > 0 {
                    mwe_scoring_index = next.index;
                }
            }

            index = next;
            x += 1;
        }

        // Add mwe
        if mwe_index > 0 && buffer.mwe_length == 0 {
            buffer.add_mwe(&mwe_index, token, &mwe, vocab);
        }

        // Add scoring mwe
        if mwe_scoring_index > 0 && buffer.mwe_scoring_length == 0 {
            buffer.add_mwe_scoring(&mwe_scoring_index, &mwe_scoring, vocab);
        }
    }

    /// Checks for a future verb phrase, adding it to the buffer if found, and returns true if successful.
    fn check_future_verb(&self, word: &String, vocab: &VocabDatabase, buffer: &mut Buffer) -> bool {
        // Get first index
        let mut index = match vocab.words.future_verbs.children.get(&word.to_lowercase()) {
            Some(r) => r,
            None => return false,
        };

        // Set variables
        let mut phrase = vec![word.to_string()];
        let mut verb_token = Token::default();
        let mut is_complete = false;

        // Loop until we can't anymore
        let mut x = 0;
        loop {
            index = match index.children.get(&buffer.words[x].to_lowercase().to_string()) {
                Some(r) => r,
                None => match index.children.get("[verb]") {
                    Some(rc) => rc,
                    None => break,
                },
            };
            phrase.push(buffer.words[x].to_string());

            // Check for verb pos label
            if let Some(chk_pos) = index.expected_verb_pos.clone() {
                let tmp_token = Token::new(&buffer.words[x], vocab);
                if POSTag::from_str(&chk_pos) != tmp_token.pos {
                    break;
                }
                verb_token = tmp_token;
            }

            if index.is_complete {
                is_complete = true;
                break;
            }
            x += 1;
        }

        // Add future verb, if found
        if is_complete && verb_token.is_verb() {
            buffer.add_future_verb(verb_token, &phrase);
            return true;
        }

        false
    }
}

impl Buffer {
    /// Creates a new Buffer instance with initialized TokenizedInput and words split from the cleaned input string.
    pub fn new(input: &str, clean_str: &str) -> Self {
        Self {
            output: TokenizedInput::new(input),
            words: clean_str.split(" ").map(|w| w.to_string()).collect::<Vec<String>>(),
            ..Default::default()
        }
    }

    /// Expands system tags for dates, times, and numerical suffixes, updating MWEs in the output and returning true if expanded.
    fn expand_system_tag(&mut self, token: &Token) -> bool {
        // Date / time period suffix
        let (mut res, mut period_word, mut period_tag) = (false, String::new(), String::new());
        if ["|date_period|", "|time_period|"].contains(&self.prev_tag.as_str())
            && (SUFFIX_PAST_DATE_PERIOD_WORDS.contains(&token.word.as_str())
                || SUFFIX_FUTURE_DATE_PERIOD_WORDS.contains(&token.word.as_str()))
        {
            period_word = format!(
                "{} {}",
                self.output.tokens.last().unwrap().inner_word,
                token.word
            );
            period_tag = if SUFFIX_PAST_DATE_PERIOD_WORDS.contains(&token.word.as_str()) {
                format!("past_{}", self.prev_tag.trim_start_matches("|"))
            } else {
                format!("future_{}", self.prev_tag.trim_start_matches("|"))
            };

        // Future / Past day_of_week or month
        } else if ["|day_of_week|", "|month|"].contains(&token.word.as_str())
            && (PREFIX_PAST_DATE_WORDS.contains(&self.prev_tag.as_str())
                || PREFIX_FUTURE_DATE_WORDS.contains(&self.prev_tag.as_str()))
        {
            let mut mwe_token = token.clone();
            mwe_token.word = if PREFIX_PAST_DATE_WORDS.contains(&self.prev_tag.as_str()) {
                format!("|past_{}", token.word.trim_start_matches("|"))
            } else {
                format!("|future_{}", token.word.trim_start_matches("|"))
            };
            mwe_token.inner_word = format!("{} {}", self.prev_tag, token.inner_word);
            res = true;

            // Set MWE in output
            let mwe = self.output.mwe.last_mut().unwrap();
            mwe.position = 0;
            mwe.token = Some(mwe_token);

        // Numerical suffix
        } else if self.prev_tag.as_str() == "|num|" && token.pos == POSTag::SYS {
            let prev = self.output.tokens.last_mut().unwrap();
            prev.index = token.index;
            prev.word = token.word.to_string();
            prev.inner_word = format!("{} {}", prev.inner_word, token.inner_word);
            prev.inner_unit = token.inner_unit.to_string();
            self.prev_tag = token.word.to_string();
            res = true;

            let inner_word = prev.inner_word.clone();
            let length = self.output.tokens.len();

            // Check for time / date period prefix
            if ["|date_period|", "|time_period|"].contains(&token.word.as_str()) && length > 1 {
                let chk_word = self.output.tokens[length - 2].word.to_string();

                if PREFIX_PAST_DATE_PERIOD_WORDS.contains(&chk_word.as_str())
                    || PREFIX_FUTURE_DATE_PERIOD_WORDS.contains(&chk_word.as_str())
                {
                    period_word = format!("{} {}", chk_word, inner_word);
                    period_tag = if PREFIX_PAST_DATE_PERIOD_WORDS.contains(&chk_word.as_str()) {
                        format!("|past_{}", token.word.trim_start_matches("|"))
                    } else {
                        format!("|future_{}", token.word.trim_start_matches("|"))
                    };
                    self.output.mwe.pop().unwrap();
                }
            }
        }

        if period_tag.is_empty() {
            return res;
        }

        // Get MWE
        let mut mwe_token = self.output.tokens.last().unwrap().clone();
        mwe_token.word = period_tag;
        mwe_token.inner_word = period_word;

        // Set MWE in output
        let mwe = self.output.mwe.last_mut().unwrap();
        mwe.position = 0;
        mwe.token = Some(mwe_token);

        res
    }

    /// Adds a token to the buffer, handling system tag expansion, MWEs, and properties like negation and possession.
    pub fn push_token(&mut self, mut token: Token) {
        // Expand system tag
        if self.expand_system_tag(&token) {
            return;
        }

        // Update token properties as necessary
        self.prev_tag = token.word.to_string();
        token.is_possessive = self.is_possessive;
        token.is_negative = self.not_position.is_some() && (token.is_verb() || token.is_noun());

        // Add have / had MWE if needed
        let mut mwe_added = false;
        if self.mwe_length == 0
            && (self.not_position.is_some()
                || self.have_position.is_some()
                || self.had_position.is_some())
            && (token.is_noun() || token.is_verb())
        {
            self.add_not_have_mwe(&token);
            mwe_added = true;
        }

        // Reset needed properties
        self.is_possessive = false;
        if (token.is_noun() || token.is_verb())
            && !["have", "has", "had"].contains(&token.word.as_str())
        {
            self.not_position = None;
            self.have_position = None;
            self.had_position = None;
        } else if token.word.as_str() == "not" {
            self.not_position = Some(self.output.tokens.len());
        } else if token.word.as_str() == "have" || token.word.as_str() == "has" {
            self.have_position = Some(self.output.tokens.len());
        } else if token.word.as_str() == "had" {
            self.had_position = Some(self.output.tokens.len());
        }
        self.output.tokens.push(token);

        // Update mwe as needed
        if self.mwe_length > 0 {
            self.mwe_length -= 1;
        } else if !mwe_added {
            self.output.mwe.push(MWE {
                position: self.output.tokens.len() - 1,
                token: None,
            });
        }

        // Update scoring mwe as needed
        if self.mwe_scoring_length > 0 {
            self.mwe_scoring_length -= 1;
        } else {
            self.output.mwe_scoring.push(MWE {
                position: self.output.tokens.len() - 1,
                token: None,
            });
        }
    }

    /// Adds an MWE with 'not', 'have', or 'had' included, updating the token and MWE list in the output.
    fn add_not_have_mwe(&mut self, verb_token: &Token) {
        // Get start position
        let start = match self.get_not_have_start_position() {
            Some(r) => r,
            None => return,
        };

        // Get phrase
        let mut phrase: Vec<String> = Vec::new();
        for token in &mut self.output.tokens[start..] {
            phrase.push(token.word.to_string());
        }

        // Define new token
        let mut token = verb_token.clone();
        token.word = format!("{} {}", phrase.join(" "), verb_token.word);

        // Set label
        if self.have_position.is_some() {
            if token.pos == POSTag::VB {
                token.pos = POSTag::VH;
            } else if token.pos == POSTag::VBZ {
                token.pos = POSTag::VHZ;
            }
        } else if self.had_position.is_some() {
            if token.pos == POSTag::VBD || token.pos == POSTag::VBN {
                token.pos = POSTag::VHP;
            } else if token.pos == POSTag::VB {
                token.pos = POSTag::VH;
            }
        }

        // Trim MWE vector, push token
        if self.output.mwe.len() >= phrase.len() {
            self.output.mwe.truncate(self.output.mwe.len() - phrase.len());
        }
        self.output.mwe.push(MWE {
            position: 0,
            token: Some(token),
        });
    }

    /// Retrieves the starting position for an MWE involving 'not', 'have', or 'had', if any.
    fn get_not_have_start_position(&self) -> Option<usize> {
        let mut values: Vec<usize> = vec![];
        if let Some(pos) = self.not_position {
            values.push(pos);
        }
        if let Some(pos) = self.have_position {
            values.push(pos);
        }
        if let Some(pos) = self.had_position {
            values.push(pos);
        }

        values.into_iter().min()
    }

    /// Prepends a token to the suffix list for later processing.
    pub fn prepend_suffix(&mut self, token: &Token) {
        self.suffix.insert(0, token.clone());
    }

    /// Pushes all suffix tokens to the output, adding corresponding MWEs and clearing the suffix list.
    pub fn push_suffix(&mut self) {
        for token in self.suffix.iter() {
            self.output.tokens.push(token.clone());

            self.output.mwe.push(MWE {
                position: self.output.tokens.len() - 1,
                token: None,
            });

            self.output.mwe_scoring.push(MWE {
                position: self.output.tokens.len() - 1,
                token: None,
            });
        }
        self.suffix.clear();
    }

    /// Adds a future verb token to the output, setting its properties and MWE based on the provided phrase.
    pub fn add_future_verb(&mut self, mut token: Token, phrase: &[String]) {
        token.word = phrase.join(" ");
        token.is_negative = phrase.contains(&"not".to_string());
        token.pos = if token.pos == POSTag::VB {
            POSTag::VF
        } else {
            POSTag::VFG
        };
        if phrase.contains(&"have".to_string()) {
            token.pos = POSTag::VHF;
        }
        self.mwe_length = phrase.len();
        self.output.mwe.push(MWE {
            position: 0,
            token: Some(token),
        });
    }

    /// Adds an MWE to the output, handling negation and possession, using the provided index and phrase.
    pub fn add_mwe(
        &mut self,
        index: &i32,
        _single_token: &Token,
        mwe: &[String],
        vocab: &VocabDatabase,
    ) {
        let mut token = Token::from_id(*index, vocab);
        //if token.is_verb() && !single_token.is_verb() {
        //return;
        //}
        token.word = mwe.join(" ");
        self.mwe_length = mwe.len();

        // Add not / have token, if needed
        if (self.not_position.is_some()
            || self.have_position.is_some()
            || self.had_position.is_some())
            && (token.is_noun() || token.is_verb())
        {
            self.add_not_have_mwe(&token);
        } else {
            self.output.mwe.push(MWE {
                position: 0,
                token: Some(token),
            });
        }
    }

    /// Adds a scoring MWE to the output, using the provided index and phrase.
    pub fn add_mwe_scoring(&mut self, index: &i32, mwe: &[String], vocab: &VocabDatabase) {
        let mut token = Token::from_id(*index, vocab);
        token.word = mwe.join(" ");
        self.mwe_scoring_length = mwe.len();

        self.output.mwe_scoring.push(MWE {
            position: 0,
            token: Some(token),
        });
    }
}
