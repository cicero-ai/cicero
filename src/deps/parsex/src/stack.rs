use crate::query::Query;
use crate::token::Token;
use crate::token_iter::TokenIter;
use regex::{Regex, RegexBuilder};
use std::collections::HashMap;
use std::iter::repeat;

#[derive(Debug, Clone)]
pub struct Stack {
    tag_id: usize,
    parent_id: usize,
    depth: HashMap<String, Vec<usize>>,
    pub tokens: HashMap<usize, Token>,
    stack: HashMap<usize, Vec<usize>>,
    code: String,
    position: usize,
    parent_position: usize,
    excludes: Vec<usize>,
}

impl Stack {
    /// Instantiate a new token stack
    pub fn new(code: &str) -> Self {
        let mut stack = Self::default();
        stack.code = code.to_owned();

        stack
    }

    /// Push new token onto stack
    pub fn push(
        &mut self,
        tag: &str,
        attr_string: &str,
        is_single: &bool,
        tag_string: &str,
    ) -> usize {
        self.tag_id += 1;
        self.stack.entry(self.parent_id).or_default().push(self.tag_id);
        if !is_single {
            self.depth.entry(tag.to_string()).or_default().push(self.tag_id);
        }

        // Add token
        let token_depth = if let Some(d) = self.depth.get(tag) { d.len() } else { 0 };
        let contents = if tag == "!" { tag_string } else { "" };
        self.tokens.insert(
            self.tag_id,
            Token::new(
                &self.tag_id,
                &self.parent_id,
                &token_depth,
                &is_single,
                tag,
                attr_string,
                contents,
            ),
        );
        self.code = self.code.replacen(tag_string, format!("<parsex{}>", self.tag_id).as_str(), 1);

        self.parent_id = self.tag_id;
        self.tag_id
    }

    /// Close a previously opened HTML tag wwithin stack
    pub fn close_tag(&mut self, tag: &str, tag_string: &str) {
        if !self.depth.contains_key(tag) {
            return;
        }

        let tag_id: usize = self.depth.get_mut(tag).unwrap().pop().unwrap();
        if self.depth.get(tag).unwrap().is_empty() {
            self.depth.remove(tag);
        }
        self.code = self.code.replacen(tag_string, format!("</parsex{}>", &tag_id).as_str(), 1);

        // Update token as necessary
        let token = self.tokens.get_mut(&tag_id).unwrap();
        token.mark_closed();
        self.parent_id = token.parent_id();

        // Get body
        //let search = format!("<parsex{}>(.*?)</parsex{}>", tag_id, tag_id);
        //let re = RegexBuilder::new(&search).dot_matches_new_line(true).build().unwrap();
        //if let Some(m) = re.captures(&self.code) {
            //token.set_contents(m.get(1).unwrap().as_str());
        //}
    }

    /// Pull the next immutable token off the stack in hierarchial order, top to bottom, left to right
    pub fn pull(&mut self) -> Option<Token> {
        // Get next token
        if let Some(pos) = self.get_next_position() {
            let token = self.get(&pos).unwrap();
            return Some(token.clone());
        }

        None
    }

    /// Pull the next mutable token off the stack in hierarchial order, top to bottom, left to right
    pub fn pull_mut(&mut self) -> Option<&mut Token> {
        // Get next token
        if let Some(pos) = self.get_next_position() {
            return Some(self.get_mut(&pos).unwrap());
        }

        None
    }

    /// Retrieve single immutable token
    pub fn get(&mut self, token_id: &usize) -> Option<Token> {
        if let Some(token) = self.tokens.get(token_id) {
            return Some(token.clone());
        }
        None
    }

    /// Retrieve single mutable token
    pub fn get_mut(&mut self, token_id: &usize) -> Option<&mut Token> {
        self.tokens.get_mut(token_id)
    }

    /// Get next position to pull token from
    fn get_next_position(&mut self) -> Option<usize> {
        // Get next position
        let mut parent_id = self.position;
        loop {
            // Get children, and check if we hit bottom of tree.
            if parent_id == 0 && !self.stack.contains_key(&parent_id) {
                return None;
            } else if !self.stack.contains_key(&parent_id) {
                parent_id = self.tokens.get(&parent_id).unwrap().parent_id();
            }
            let mut children = self.stack.get(&parent_id).unwrap().clone();
            children.sort();

            // Get next index
            let mut index = match children.binary_search(&self.position) {
                Ok(r) => r + 1,
                Err(_) => 0,
            };

            // Check excludes
            if index < children.len() && self.excludes.contains(&children[index]) {
                index += 1;
            }

            // Check if done this set of tokens
            if index >= children.len()
                && ((parent_id == 0)
                    || (self.parent_position > 0 && parent_id == self.parent_position))
            {
                return None;
            } else if index >= children.len() {
                self.position = parent_id;
                parent_id = self.tokens.get(&parent_id).unwrap().parent_id();
                continue;
            }
            self.position = children[index];
            break;
        }

        Some(self.position)
    }

    /// Save token
    pub fn save(&mut self, token: &Token) {
        if let Some(item) = self.tokens.get_mut(&token.id()) {
            *item = token.clone();
        }
    }
    /// Iterate over all tokens in stack in hierarchial order, top to bottom, left to right
    pub fn iter(&mut self) -> TokenIter {
        Query::new(self).iter()
    }
    // Query tags by desired criteria
    pub fn query(&mut self) -> Query {
        Query::new(self)
    }

    /// Get contents of tag
    pub fn get_contents(&mut self, token_id: &usize) -> Option<String> {

        // Get body
        let search = format!(r"(?s)<parsex{}>(.*?)</parsex{}>", token_id, token_id);
        let re = RegexBuilder::new(&search).dot_matches_new_line(true).build().unwrap();
        if let Some(m) = re.captures(&self.code) {
            return Some(m.get(1).unwrap().as_str().to_string());
        }
        None
    }

    /// Set contents of tag
    pub fn set_contents(&mut self, token_id: &usize, new_contents: &str) {

        // Initialize
        let search = format!(r"(?s)<parsex{}>.*?</parsex{}>", token_id, token_id);
        let replace_text = format!("<parsex{}>{}</parsex{}>", token_id, new_contents, token_id);

        // Search and replace
        let re = RegexBuilder::new(&search).dot_matches_new_line(true).build().unwrap();
        self.code = re.replace_all(&self.code.clone(), &replace_text).to_string();
    }


    // Get children tokens, must call .iter() or .to_vec() on this result
    pub fn get_children(&mut self, token_id: &usize) -> Query {
        self.query().parent_id(token_id)
    }

    /// Set parent id for next pull of all tokens
    pub fn set_parent_position(&mut self, parent_id: &usize) {
        self.position = *parent_id;
        self.parent_position = *parent_id;
    }

    /// Set excludes
    pub fn set_excludes(&mut self, excludes: &Vec<usize>) {
        self.excludes = excludes.clone();
    }

    // Render stack
    pub fn render(&mut self) -> String {
        self.render_tag(&0)
    }

    /// Old render method, deprecated, leaving here in case of problems with new methodology.
    pub fn render_old(&mut self) -> String {
        // Go through  stack
        let mut html = self.code.clone();
        while let Some(token) = self.pull() {
            // Get attribute string
            let attr_string: String = token
                .attributes()
                .iter()
                .map(|(key, value)| format!("{}=\"{}\"", key, value))
                .collect::<Vec<String>>()
                .join(" ");

            // Get replace string
            let open_tag = format!("<{} {} {}", token.tag(), attr_string, token.attr_extra());
            let replace_str = if token.is_closed() {
                format!(
                    "{}>{}</{}>",
                    open_tag.trim_end(),
                    token.contents(),
                    token.tag()
                )
            } else {
                format!("{}/>", open_tag)
            };

            // Quick replace if comment or tag not closed
            if token.tag() == "!" {
                let search = format!("<parsex{}>", token.id());
                html = html.replace(&search, token.contents().as_str());
                continue;
            } else if !token.is_closed() {
                let search = format!("<parsex{}>", token.id());
                html = html.replace(&search, &replace_str);
                continue;
            }

            // Replace open / close tag
            let search = format!(r"(?s)<parsex{}>.*?</parsex{}>", token.id(), token.id());
            let re = RegexBuilder::new(&search)
                .dot_matches_new_line(true)
                .build()
                .unwrap();
            html = re.replace_all(&html, &replace_str).to_string();
        }

        html
    }

    /// Render tag
    pub fn render_tag(&mut self, token_id: &usize) -> String {

        // Get contents
        let mut html = if *token_id == 0 {
            self.code.clone()
        } else {
            self.get_contents(&token_id).unwrap_or("".to_string())
        };
        if html.is_empty() {
            return html;
        }

        // Go through tokenized tags
        let re = Regex::new(r"<([\/]?)parsex(\d+?)>").unwrap();
        for cap in re.captures_iter(&html.clone()) {

            // Set variables
            let is_closing: bool = cap.get(1).unwrap().as_str() == "/";
            let token_id = cap.get(2).unwrap().as_str().parse::<usize>().unwrap();

            // Get token
            let token = match self.tokens.get(&token_id) {
                Some(r) => r,
                None => continue
            };

            // Get attribute string
            let attr_string: String = token.attributes().iter().map(|(key, value)| format!("{}=\"{}\"", key, value)).collect::<Vec<String>>().join(" ");
            let single_slash = if !token.is_closed() { "/" } else { "" };
        let tag_string = format!("{} {} {}", attr_string, token.attr_extra(), single_slash);

            let open_tag = format!("<{} {}>", token.tag(), tag_string.trim());
            html = html.replace(format!("<parsex{}>", token.id()).as_str(), open_tag.as_str());

            let close_tag = format!("<{}>", token.tag());
            let search_close = format!("<parsex{}>", token.id());
            html = html.replace(&search_close.as_str(), &close_tag.as_str());
        }

        html.to_string()
    }

    /// Clone stack from starting tag (eg. body, nav menu, footer) to extract certain portion of page.
    pub fn clone_from(&mut self, token_id: &usize, excludes: &Vec<usize>) -> Option<Stack> {

        // Get body contents
        let code = match self.get_contents(&token_id) {
            Some(r) => r,
            None => return None
        };

        // Start stack
        let mut res = Stack::new("");
        res.code = code;

        // Go through tokens
        for tag in self.query().parent_id(&token_id).excludes(&excludes).iter() {
            res.stack.entry(tag.parent_id()).or_default().push(tag.id());
            if !tag.is_self_closing() {
                res.depth.entry(tag.tag()).or_default().push(tag.id());
            }

            // Add token
            let token_depth = if let Some(d) = res.depth.get(&tag.tag()) { d.len() } else { 0 };
            let contents: String = if tag.tag() == "!" { tag.contents() } else { String::new() };
            self.tokens.insert(tag.id(), tag.clone());
        }

        Some(res)
    }

    // Rebuild stack from scratch with proper line spacing and indentation for messy HTML code, or pages all on one-line from React.
    pub fn rebuild(&mut self) -> String {

        let mut html = self.code.clone();
        let same_line_tags = vec!["i", "b", "a", "center", "h1", "h2", "h3", "h4", "h5", "h6", "td", "li"];
        let mut parents = Vec::new();
        let mut parent_tags: Vec<String> = Vec::new();

        // Delete comments
        let re = RegexBuilder::new(r"(?s)<!--(.*?)-->")
            .dot_matches_new_line(true)
            .build()
            .unwrap();
        html = re.replace_all(&html, "").to_string();

        // Go through stack
        for token in self.iter() {

            // Comment
            if token.tag() == "!" {
                let search = format!("<parsex{}>", token.id());
                html = html.replace(&search.as_str(), "");
                continue;
            }

            // Scroll up, if needed
            if parents.len() > 0 && *parents.last().unwrap() != token.parent_id() {
                while parents.len() > 0 {
                    if parents.len() > 0 && *parents.last().unwrap() == token.parent_id() {
                        break;
                    } else {
                        parents.pop();
                        parent_tags.pop();
                    }
                }
            }

            // Get attribute string
            let attr_string: String = token.attributes().iter().map(|(key, value)| format!("{}=\"{}\"", key, value)).collect::<Vec<String>>().join(" ");
            let single_slash = if !token.is_closed() { "/" } else { "" };
        let tag_string = format!("{} {} {}", attr_string, token.attr_extra(), single_slash);

            // Get indent and newline
            let mut indent = String::new();
            if parents.len() > 0 && !same_line_tags.contains(&parent_tags.last().unwrap().as_str()) { 
                indent = repeat(' ').take(parents.len() * 4).collect::<String>();
            }
            let suffix = if same_line_tags.contains(&token.tag().as_str()) { "" } else { "\n" };

            // Opening tag
            let open_tag = format!("{}<{} {}>{}", indent, token.tag(), tag_string.trim(), suffix);
            html = html.replace(format!("<parsex{}>", token.id()).as_str(), open_tag.as_str());

            // Closing tag
            let close_tag = format!("</{}>", token.tag());
            let search_close = format!("</parsex{}>", token.id());
            html = html.replace(&search_close.as_str(), &close_tag.as_str());

            if token.is_closed() && !same_line_tags.contains(&token.tag().as_str()) {
                parents.push(token.id().clone());
                parent_tags.push(token.tag().clone().to_string());
            }
        }

        html = html.replace("\n\n", "\n");
        html
    }

}

impl Default for Stack {
    fn default() -> Stack {
        Stack {
            tag_id: 0,
            parent_id: 0,
            depth: HashMap::new(),
            tokens: HashMap::new(),
            stack: HashMap::new(),
            code: String::new(),
            position: 0,
            parent_position: 0,
            excludes: Vec::new()
        }
    }

}

