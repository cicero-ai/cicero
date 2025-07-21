
use regex::Regex;
use std::collections::HashMap;
use serde_derive::{Serialize, Deserialize};
use parsex::{Stack, Token};
use super::{HtmlTable, HtmlForm};
use std::iter::repeat;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Markdown {
    pub headings: HashMap<u8, String>,
    pub forms: Vec<HtmlForm>,
    pub tables: Vec<HtmlTable>,
    pub code: String
}

impl Markdown {

    /// Create markdown from page
    pub fn generate(title: &str, stack: &mut Stack) -> Self {

        // Start markdown
        let mut markdown = Self {
            headings: HashMap::new(),
            forms: Vec::new(),
            tables: Vec::new(),
            code: format!("\n# {}\n\n", title)
        };

        // Parse stack and generate markdown
        markdown.parse_stack(stack);
        markdown
    }

    /// Parse stack and generate markdown
    fn parse_stack(&mut self, stack: &mut Stack) {

        // Initialize
        let re_header = Regex::new(r"h(\d)").unwrap();

        // Go through stack
        for tag in stack.iter() {

            // Heading
            if let Some(header_caps) = re_header.captures(&tag.tag()) {
                let hnum: u8 = header_caps.get(1).unwrap().as_str().parse::<u8>().unwrap();
                let entry = self.headings.entry(hnum.clone()).or_insert_with(String::new);
                entry.push_str(&tag.contents().clone());

                self.code.push_str(format!("{} {}\n\n", repeat('#').take(hnum as usize).collect::<String>(), tag.contents()).as_str());

        // List (ordered / unordered)
            } else if tag.tag() == "ul".to_string() || tag.tag() == "ol".to_string() {
                let is_ordered = if tag.tag() == "ol".to_string() { true } else { false };
                let list_code = self.from_list(&tag, stack, is_ordered, 0);
                self.code.push_str(format!("{}\n", list_code).as_str());

            // Paragraph
            } else if tag.tag() == "p".to_string() {
                self.code.push_str(format!("{}\n\n", tag.strip_tags()).as_str());
            }
        }
    }

    /// Format list into markdown
    fn from_list(
        &self, 
        parent_tag: &Token, 
        stack: &mut Stack, 
        is_ordered_list: bool,
        depth: usize
    ) -> String {

        if parent_tag.attr_contains("class", "nav") {
            return "".to_string();
        }

        // Initialize
        let indent = repeat(' ').take(depth * 4).collect::<String>();
        let mut code = String::new();
        let mut num_item = 1;

        // Go through child tags
        for tag in parent_tag.children(stack).iter() {

            if tag.tag() == "li".to_string() {
                let prefix = if is_ordered_list { format!("{}.", num_item) } else { "*".to_string() };
                code.push_str(format!("{}{} {}\n", indent, prefix, tag.strip_tags()).as_str());
                num_item += 1;

            } else if tag.tag() == "ul".to_string() || tag.tag() == "ol".to_string() {
                let is_ordered = if tag.tag() == "ol".to_string() { true } else { false };
                let list_code = self.from_list(&tag, stack, is_ordered, (depth + 1));
                code.push_str(&list_code.as_str());

            }
        }

        code
    }

}

impl Default for Markdown {
    fn default() -> Markdown {
        Markdown {
            headings: HashMap::new(),
            forms: Vec::new(),
            tables: Vec::new(),
            code: String::new()
        }
    }
}

