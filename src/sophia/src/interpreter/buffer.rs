use super::{AntecedentBuffer, CoreferenceCategories, Phrase};
use crate::pos_tagger::POSTag;
use crate::tokenizer::Token;
use std::fmt;

/// A buffer for processing tokens, tracking verbs, nouns, pronouns, and antecedents, with support for phrase splitting and enclosed character handling.
#[derive(Default)]
pub struct Buffer {
    pub position: usize,
    pub tokens: Vec<Token>,
    pub is_locked: bool,
    pub enclosed_chars: Vec<char>,
    pub enclosed_chars_num_phrases: usize,
    pub verbs: Vec<usize>,
    pub nouns: Vec<usize>,
    pub pronouns: Vec<usize>,
    pub antecedents: AntecedentBuffer,
}

static ENCLOSED_START_CHARS: &[char] = &['"', '\'', '(', '[', '<', '|'];

impl Buffer {
    /// Creates a new Buffer instance with an initialized AntecedentBuffer using the provided coreference categories.
    pub fn new(coref: &CoreferenceCategories) -> Self {
        Self {
            antecedents: AntecedentBuffer::new(coref),
            ..Default::default()
        }
    }

    /// Adds a token to the buffer, updating verb, noun, pronoun, and antecedent tracking, and returns the token's index.
    pub fn add(&mut self, buf_token: &Token) -> usize {
        let mut token = buf_token.clone();

        // Process token type
        if token.is_verb() {
            self.verbs.push(self.tokens.len());
            if self.verbs.len() == 1 {
                self.position = self.tokens.len();
            }
            self.is_locked = false;

        // Add noun
        } else if token.is_noun() {
            self.nouns.push(self.tokens.len());
            self.is_locked = false;
            self.antecedents.add_noun(&token);

        // Add pronoun
        } else if token.is_pronoun() {
            self.pronouns.push(self.tokens.len());
            self.antecedents.resolve_pronoun(&mut token);
        }

        // Add non-pronoun to antecedent buffer
        if !token.is_pronoun() {
            self.antecedents.add_non_noun(&token);
        }

        // Add token
        self.tokens.push(token);
        self.tokens.len() - 1
    }

    /// Checks if the buffer can be split at the given position based on token type, enclosed characters, and buffer state.
    pub fn can_split(&self, x: usize) -> bool {
        if self.tokens[x].pos == POSTag::SYM
            && self.enclosed_chars.is_empty()
            && ENCLOSED_START_CHARS.contains(&self.tokens[x].word.chars().next().unwrap())
        {
            return true;
        } else if self.tokens[x].pos == POSTag::SYM
            && !self.enclosed_chars.is_empty()
            && self.enclosed_chars[1] == self.tokens[x].word.chars().next().unwrap()
        {
            return true;
        } else if !self.pronouns.is_empty()
            && !self.verbs.is_empty()
            && self.tokens[x].pos == POSTag::CC
        {
            return true;
        } else if self.verbs.len() < 2 || self.is_locked {
            return false;
        } else if self.nouns.is_empty() {
            return false;
        }

        true
    }
    /// Attempts to split the buffer into a Phrase if conditions are met, determining the split position and handling enclosed characters.
    pub fn split(&mut self) -> Option<Phrase> {
        // Check minimum requirements
        if !self.can_split(self.tokens.len() - 1) {
            return None;
        }

        // Determine split position
        let mut split_pos = None;
        for x in self.position..self.tokens.len() {
            self.position = x + 1;

            // Check enclosed char
            if self.tokens[x].pos == POSTag::SYM
                && self.enclosed_chars.is_empty()
                && ENCLOSED_START_CHARS.contains(&self.tokens[x].word.chars().next().unwrap())
            {
                self.enclosed_chars = vec![self.tokens[x].word.chars().next().unwrap(), ' '];
                self.enclosed_chars_num_phrases = 0;
                self.enclosed_chars[1] = match self.enclosed_chars[0] {
                    '\'' => '\'',
                    '(' => ')',
                    '[' => ']',
                    '{' => '}',
                    '<' => '>',
                    '|' => '|',
                    _ => '"',
                };
                split_pos = Some(x);
                break;
            } else if self.tokens[x].pos == POSTag::SYM
                && self.enclosed_chars.len() == 2
                && self.enclosed_chars[1] == self.tokens[x].word.chars().next().unwrap()
            {
                split_pos = Some(x);
                break;
            }

            // Unlock, if needed
            if self.is_locked && (self.tokens[x].is_noun() || self.tokens[x].is_verb()) {
                self.is_locked = false;
            }

            if (!self.nouns.is_empty() && self.nouns[0] >= x) || self.is_locked {
                continue;
            } else if self.tokens[x].is_sentence_stopper() {
                split_pos = Some(x);
                break;
            }

            // Lock if CC tag
            if self.tokens[x].pos == POSTag::CC {
                self.is_locked = true;
                continue;
            }

            // Handle previous comma
            if x > 0 && self.tokens[x - 1].word.as_str() == "," {
                if ["CS", "CA", "VBG"].contains(&self.tokens[x].pos.to_str().as_str()) {
                    self.is_locked = true;
                    continue;
                } else {
                    split_pos = Some(x - 1);
                    break;
                }
            }

            // Check for potential phrase splitter
            if self.tokens[x].word.as_str() == ","
                || !["CC", "DT", "PRP", "RB", "RBS", "RBR"]
                    .contains(&self.tokens[x].pos.to_str().as_str())
            {
                continue;
            }

            // If adjective, ensure following token is noun
            if self.tokens[x].is_adjective() && (self.check_determiner_offset(x) > 0) {
                continue;
            } else if x > 0
                && (self.tokens[x].is_pronoun() || self.tokens[x].is_determiner())
                && self.tokens[x - 1].is_preposition()
            {
                self.is_locked = true;
                continue;
            } else if self.tokens[x].is_adverb() && x > 0 && self.tokens[x - 1].is_verb() {
                continue;
            }

            split_pos = Some(x);
            break;
        }

        // Split if needed
        if let Some(pos) = split_pos {
            let phrase = self.do_split(pos);
            return Some(phrase);
        }

        None
    }

    /// Performs the actual split at the specified position, creating a new Phrase and updating the buffer's token list.
    pub fn do_split(&mut self, split_pos: usize) -> Phrase {
        // Get phrase
        let remaining_tokens = self.tokens.split_off(split_pos);
        let phrase = Phrase::new(&0, None);

        // Drain buffer after a split
        self.tokens = remaining_tokens;
        //self.drain(phrase.tokens.len());

        phrase
    }

    /// Drains the buffer after a split, updating verb, noun, and pronoun indices and resetting position and lock state.
    pub fn drain(&mut self, length: usize) {
        self.verbs = self.verbs.iter().filter(|&&v| v > length).map(|&v| v - length).collect();
        self.nouns = self.nouns.iter().filter(|&&v| v > length).map(|&v| v - length).collect();
        self.pronouns =
            self.pronouns.iter().filter(|&&v| v > length).map(|&v| v - length).collect();
        self.position = if !self.verbs.is_empty() {
            self.verbs[0]
        } else {
            0
        };
        self.is_locked = false;
    }

    /// Checks for a determiner followed by a noun (with optional adjective) and returns the offset (2 or 3) or 0 if not found.
    pub fn check_determiner_offset(&self, pos: usize) -> usize {
        if self.tokens.len() < (pos + 1) {
            return 0;
        }

        if self.tokens[pos + 1].is_noun() {
            return 2;
        } else if self.tokens.len() >= (pos + 2)
            && self.tokens[pos + 1].is_adjective()
            && self.tokens[pos + 2].is_noun()
        {
            return 3;
        }

        0
    }
}

impl fmt::Debug for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let words = self.tokens.iter().map(|token| token.word.to_string()).collect::<Vec<String>>();
        let verbs = self
            .verbs
            .iter()
            .map(|pos| format!("{} {}", self.tokens[*pos].word, pos))
            .collect::<Vec<String>>();
        let nouns = self
            .nouns
            .iter()
            .map(|pos| self.tokens[*pos].word.to_string())
            .collect::<Vec<String>>();
        write!(
            f,
            "[buffer] {} [verbs] {} [nouns] {}",
            words.join(" "),
            verbs.join(", "),
            nouns.join(", ")
        )
    }
}
