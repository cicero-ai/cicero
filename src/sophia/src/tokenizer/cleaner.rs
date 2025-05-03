use super::{Buffer, Token};
use crate::vocab::VocabDatabase;

static SPECIAL_CHARS: &[char] = &[
    '~', '`', '!', '@', '#', '$', '%', '&', '*', '(', ')', '-', '_', '+', '[', ']', '{', '}', '\\',
    '|', ';', ':', '\'', '"', ',', '.', '<', '>',
];
static NUMERIC_CHARS: &[char] = &['.', ',', '^', '*', '/', ':'];

/// A utility for cleaning and classifying tokens, tracking character properties like numeric status and special characters.
#[derive(Default)]
pub struct TokenCleaner {
    chars: Vec<char>,
    word_len: usize,
    numeric_len: usize,
    pub is_numeric: bool,
    pub has_decimal: bool,
    pub has_special: bool,
}

impl TokenCleaner {
    /// Creates a new TokenCleaner instance with default values, marking it as numeric.
    pub fn new() -> Self {
        Self {
            is_numeric: true,
            ..Default::default()
        }
    }

    /// Resets the TokenCleaner to its initial state, equivalent to calling `new`.
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// Cleans and classifies a word, updating the buffer with tokens for prefixes, suffixes, or special cases, returning the cleaned word if applicable.
    pub fn clean(
        &mut self,
        mut word: String,
        vocab: &VocabDatabase,
        buffer: &mut Buffer,
    ) -> Option<String> {
        // Scan characters
        self.scan_chars(&mut word, vocab, buffer);

        // Classify numeric
        if self.is_numeric {
            self.classify_numeric(&word, vocab, buffer);
            return None;
        }

        self.classify_token(&word, vocab, buffer)
    }

    /// Scans characters in a word, stripping prefixes/suffixes, updating possessive status, and tracking numeric/special properties.
    fn scan_chars(&mut self, word: &mut String, vocab: &VocabDatabase, buffer: &mut Buffer) {
        // Check for possession
        if word.ends_with("'s") {
            buffer.is_possessive = true;
            *word = word[..word.len() - 2].to_string();
        }

        // Iterate through chars
        let mut in_prefix = true;
        for (x, c) in word.chars().enumerate() {
            // Check for currency
            if x == 0 && (c == '-' || c == '+' || c.is_currency_symbol()) {
                self.chars.push(c);

            // Prefix symbol
            } else if in_prefix && SPECIAL_CHARS.contains(&c) {
                buffer.push_token(Token::prefix(&c.to_string(), vocab));
            } else {
                in_prefix = false;

                // Update word ending position
                if c.is_alphanumeric() {
                    self.word_len = self.chars.len() + 1;
                }

                // Check if it's numeric
                if self.is_numeric && !self.check_numeric(c) {
                    self.is_numeric = false;
                    self.numeric_len = self.chars.len();
                }
                self.chars.push(c);
            }
        }
    }

    /// Classifies a numeric word as a time or general numeric token, pushing it to the buffer.
    fn classify_numeric(&mut self, word: &String, vocab: &VocabDatabase, buffer: &mut Buffer) {
        if self.is_time() {
            buffer.push_token(Token::special(word, "|time|", "", "", vocab));
        } else {
            buffer.push_token(Token::numeric(word, vocab));
        }
    }

    /// Classifies a non-numeric token, handling numeric suffixes (e.g., decades, ordinals) and returning the cleaned word or None if fully processed.
    fn classify_token(
        &mut self,
        word: &String,
        vocab: &VocabDatabase,
        buffer: &mut Buffer,
    ) -> Option<String> {
        // Check for numeric with suffix (eg. 3rd, 90s).
        if self.numeric_len > 0 {
            let suffix: String = self.chars[self.numeric_len..].iter().collect();

            if self.is_decade(&suffix) {
                let value = self.chars[..self.numeric_len].iter().collect::<String>();
                buffer.push_token(Token::special(
                    word,
                    "|date_period|",
                    &value,
                    &suffix,
                    vocab,
                ));
                return None;
            } else if let Some((suffix_tag, _)) = vocab.preprocess.hashes.get(&suffix) {
                let value = self.chars[..self.numeric_len].iter().collect::<String>();
                buffer.push_token(Token::special(word, suffix_tag, &value, &suffix, vocab));
                return None;
            }
        }

        // Add suffix to buffer
        for c in self.chars[self.word_len..].iter() {
            buffer.prepend_suffix(&Token::suffix(&c.to_string(), vocab));
        }

        if self.word_len > 0 {
            Some(self.chars[..self.word_len].iter().collect())
        } else {
            None
        }
    }

    /// Checks if a character maintains the numeric status of a word, updating decimal and special character flags.
    fn check_numeric(&mut self, c: char) -> bool {
        let mut ok = false;

        // Digit
        if c.is_ascii_digit() {
            ok = true;

        // Special  numeric character (. , /, etc.)
        } else if NUMERIC_CHARS.contains(&c) {
            // Only one non-comma character allowed
            if (self.has_decimal || self.has_special) && c != ',' {
                self.is_numeric = false;
                return false;
            }

            // Decimal or other character?
            if c == '.' {
                self.has_decimal = true;
            } else if c != ',' {
                self.has_special = true;
            }
            ok = true;
        }

        ok
    }

    /// Determines if the character sequence represents a time format (e.g., H:MM or HH:MM).
    pub fn is_time(&self) -> bool {
        let res = &self.chars;

        // Check for H:MM
        if res.len() == 4 {
            if res[1] != ':' || res[0] == '0' {
                return false;
            }

            // Ensure  valid minutes
            match format!("{}{}", res[2], res[3]).parse::<u8>() {
                Ok(mins) => {
                    if mins > 59 {
                        return false;
                    }
                }
                Err(_) => return false,
            };

        // Check for HH:MM
        } else if res.len() == 5 {
            if res[2] != ':' || !['0', '1', '2'].contains(&res[0]) {
                return false;
            }

            // Ensure  valid minutes
            match format!("{}{}", res[3], res[4]).parse::<u8>() {
                Ok(mins) => {
                    if mins > 59 {
                        return false;
                    }
                }
                Err(_) => return false,
            };
        } else {
            return false;
        }

        true
    }

    /// Checks if the character sequence represents a decade (e.g., 90s or 1990s).
    fn is_decade(&self, suffix: &String) -> bool {
        if suffix.as_str() != "s" {
            return false;
        }
        let res = &self.chars;

        // Check for 2 or 4 digit year

        (res.len() == 3 && res[1] == '0')
            || (res.len() == 5 && res[3] == '0' && (res[0] == '1' || res[0] == '2'))
    }
}

/// A trait for checking if a character is a currency symbol.
trait IsCurrencySymbol {
    fn is_currency_symbol(self) -> bool;
}

impl IsCurrencySymbol for char {
    /// Implements `IsCurrencySymbol` for `char`, checking if the character is a currency symbol ($, €, £, ¥).
    fn is_currency_symbol(self) -> bool {
        match self {
            '$' | '€' | '£' | '¥' => true,
            _ => false,
        }
    }
}
