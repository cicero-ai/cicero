
use regex::Regex;
use super::parse_attr;
use crate::query::Query;
use crate::stack::Stack;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    id: usize,
    parent_id: usize,
    is_closed: bool,
    is_self_closing: bool,
    depth: usize,
    tag: String,
    attributes: HashMap<String, String>,
    attr_extra: String,
    contents: String,
}

impl Token {
    /// Create new token instance
    pub fn new(
        id: &usize,
        parent_id: &usize,
        depth: &usize,
        is_self_closing: &bool,
        tag: &str,
        attr_string: &str,
        contents: &str,
    ) -> Self {
        let (attr, attr_extra) = parse_attr(attr_string);

        Self {
            id: *id,
            parent_id: *parent_id,
            is_closed: false,
            is_self_closing: is_self_closing.clone(),
            depth: *depth,
            tag: tag.to_string(),
            attributes: attr,
            attr_extra,
            contents: contents.to_string(),
        }
    }

    /// Get token id
    pub fn id(&self) -> usize {
        self.id
    }

    /// Get token's parent id
    pub fn parent_id(&self) -> usize {
        self.parent_id
    }

    /// Get if token has closing tag
    pub fn is_closed(&self) -> bool {
        self.is_closed
    }

    /// Get if token is self closing
    pub fn is_self_closing(&self) -> bool {
        self.is_self_closing
    }

    // Get depth
    pub fn depth(&self) -> usize {
        self.depth
    }

    /// Get name of HTML tag
    pub fn tag(&self) -> String {
        self.tag.clone()
    }

    /// Get attributes
    pub fn attributes(&self) -> HashMap<String, String> {
        self.attributes.clone()
    }

    /// Get extra non-attribute text within opening tag
    pub fn attr_extra(&self) -> String {
        self.attr_extra.clone()
    }

    /// Get single attribute value
    pub fn attr(&self, key: &str) -> Option<String> {
        let res = match self.attributes.get(&key.to_string()) {
            Some(r) => r,
            None => {
                return None;
            }
        };
        Some(res.clone())
    }

    /// Check if attribute has key, and if key equals to value
    pub fn attr_equals(&self, key: &str, value: &str) -> bool {
        if (!self.attributes.contains_key(&key.to_string()))
            || (self.attributes.get(&key.to_string()).unwrap() != value)
        {
            return false;
        }
        true
    }

    /// Check if attribute has key, and if key contans value
    pub fn attr_contains(&self, key: &str, value: &str) -> bool {
        if (!self.attributes.contains_key(&key.to_string()))
            || (!self.attributes.get(&key.to_string()).unwrap().contains(&value))
        {
            return false;
        }
        true
    }

    /// Check if attribute has key, and if key contains value
    pub fn attr_has_segment(&self, key: &str, value: &str) -> bool {
        let input = match self.attributes.get(&key.to_string()) {
            Some(r) => r,
            None => return false
        };

        let chk: Vec<&str> = input.split(" ").collect();
        chk.contains(&value)
    }

    // Check if has attribute
    pub fn has_attr(&self, key: &str) -> bool {
        self.attributes.contains_key(&key.to_string())
    }

    /// Get contents between start and closing tags.  Blank string if tag not closed.
    pub fn contents(&self) -> String {
        self.contents.trim().to_string()
    }

    /// Get contents with all HTML tags stripped
    pub fn strip_tags(&self) -> String {
        let re = Regex::new(r"<([\/]?)parsex(\d+)>").unwrap();
        re.replace_all(&self.contents, "").to_string()
    }



    /// Get mutable instance of tag
    pub fn as_mut<'a>(&self, stack: &'a mut Stack) -> &'a mut Token {
        stack.get_mut(&self.id).unwrap()
    }

    /// Get children, returns query so must call .iter() or .to_vec() on results.
    pub fn children<'b>(&'b self, stack: &'b mut Stack) -> Query {
        stack.get_children(&self.id)
    }

    /// Mark token as closed
    pub fn mark_closed(&mut self) {
        self.is_closed = true;
    }

    /// Mark opened
    pub fn mark_opened(&mut self) {
        self.is_closed = false;
    }

    /// Mark self closing
    pub fn mark_self_closing(&mut self) {
        self.is_self_closing = true;
    }

    /// Mark mpt self closing
    pub fn mark_not_self_closing(&mut self) {
        self.is_self_closing = false;
    }

    /// Set tag name
    pub fn set_tag(&mut self, tag_name: &str) {
        self.tag = tag_name.to_string();
    }

    /// Update existing attribute value, or add new attribute if not exists
    pub fn set_attr(&mut self, key: &str, value: &str) {
        *self
            .attributes
            .entry(key.to_string())
            .or_insert(value.to_string()) = value.to_string();
    }

    /// Delete attribute
    pub fn del_attr(&mut self, key: &str) {
        self.attributes.remove(&key.to_string());
    }

    /// Purge all attributes
    pub fn purge_attr(&mut self) {
        self.attributes.clear();
    }

    /// Update extra non-attribute text within opening tag
    pub fn set_attr_extra(&mut self, extra: &str) {
        self.attr_extra = extra.to_string();
    }
    /// Set contents between start and closing tags.
    pub fn set_contents(&mut self, contents: &str) {
        self.contents = contents.to_string();
    }
}

