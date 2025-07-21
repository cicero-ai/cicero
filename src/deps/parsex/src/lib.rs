#![allow(warnings)]
use regex::{Regex, RegexBuilder};
use std::collections::HashMap;
pub use self::stack::Stack;
pub use self::token::Token;

pub mod query;
pub mod stack;
pub mod token;
pub mod token_iter;


/// Parse block of HTML code into a token stack
pub fn parse_html(html: &str) -> Stack {
    // Start token stack
    let mut stack = Stack::new(html);

    // Extract comments
    let re = RegexBuilder::new(r"(?s)<!--(.*?)-->")
        .dot_matches_new_line(true)
        .build()
        .unwrap();
    for cap in re.captures_iter(html) {
        let tag_string = cap.get(0).unwrap().as_str();
        stack.push("!", "", &true, tag_string);
    }

    // Go through tags
    let re = Regex::new(r"<([\/]?)(.*?)([\/]?)>").unwrap();
    for cap in re.captures_iter(html) {
        // Set variables
        let is_closing: bool = cap.get(1).unwrap().as_str() == "/";
        let mut tag = cap.get(2).unwrap().as_str().trim();
        let is_single: bool = cap.get(3).unwrap().as_str() == "/";
        let tag_string = cap.get(0).unwrap().as_str();

        // Skip if needed
        if tag.starts_with('!') {
            continue;
        }

        // Get attr string, if needed
        let mut attr_string = "";
        if let Some(cindex) = tag.find(' ') {
            attr_string = tag[cindex + 1..].trim();
            tag = &tag[..cindex];
        }

        // Process tag
        if is_closing {
            stack.close_tag(tag, tag_string);
        } else {
            stack.push(tag, attr_string, &is_single, tag_string);
        }
    }

    stack
}

/// Parse string into hashmap of attributes
pub fn parse_attr(attr_string: &str) -> (HashMap<String, String>, String) {
    // Initialize
    let mut attr = HashMap::new();
    let mut attr_extra: String = attr_string.to_string();

    // Pares attributes
    let re = Regex::new(r#"([a-zA-Z0-9_\-]+?)=(.+?)(\"|\'|#)"#).unwrap();
    for cap in re.captures_iter(attr_string) {
        let key = cap.get(1).unwrap().as_str().trim();
        let value = cap
            .get(2)
            .unwrap()
            .as_str()
            .trim_start_matches('\'')
            .trim_start_matches('"')
            .trim();
        attr.insert(key.to_string(), value.to_string());
        attr_extra = attr_extra.replace(cap.get(0).unwrap().as_str(), "");
    }

    (attr, attr_extra.trim().to_string())
}
