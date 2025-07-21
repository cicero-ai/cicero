use crate::stack::Stack;
use crate::token::Token;
use crate::token_iter::TokenIter;

pub struct Query<'a> {
    stack: &'a mut Stack,
    criteria: SearchCriteria,
}

#[derive(Debug, Clone)]
pub struct SearchCriteria {
    pub parent_id: usize,
    pub tag: String,
    pub id: String,
    pub class: String, 
    pub attr_key: String,
    pub attr_value: String,
    pub attr_contains_key: String,
    pub attr_contains_value: String,
    pub contents: String,
    pub contents_contains: String,
    pub excludes: Vec<usize>
}

impl Query<'_> {
    /// Instantiate new query
    pub fn new(stack: &mut Stack) -> Query {
        let criteria = SearchCriteria {
            parent_id: 0,
            tag: String::new(),
            id: String::new(),
            class: String::new(),
            attr_key: String::new(),
            attr_value: String::new(),
            attr_contains_key: String::new(),
            attr_contains_value: String::new(),
            contents: String::new(),
            contents_contains: String::new(),
            excludes: Vec::new(),
        };

        Query { stack, criteria }
    }

    /// Search by parent id
    pub fn parent_id(mut self, parent_id: &usize) -> Self {
        self.criteria.parent_id = *parent_id;
        self
    }

    // Search by tag name
    pub fn tag(mut self, tag: &str) -> Self {
        self.criteria.tag = tag.to_string();
        self
    }

    // Search by 'id' attribute
    pub fn id(mut self, id: &str) -> Self {
        self.criteria.id = id.to_string();
        self
    }

    // Search by 'class' attribute
    pub fn class(mut self, class: &str) -> Self {
        self.criteria.class = class.to_string();
        self
    }

    // Search by any attribute
    pub fn attr(mut self, key: &str, value: &str) -> Self {
        self.criteria.attr_key = key.to_string();
        self.criteria.attr_value = value.to_string();
        self
    }

    /// Search that an attribute contains some text
    pub fn attr_contains(mut self, key: &str, value: &str) -> Self {
        self.criteria.attr_contains_key = key.to_string();
        self.criteria.attr_contains_value = value.to_string();
        self
    }

    // Search by contents between open / closing tags
    pub fn contents(mut self, contents: &str) -> Self {
        self.criteria.contents = contents.to_string();
        self
    }

    // Search by whether or not contents between start / closing tags contains specific text.
    pub fn contents_contains(mut self, search: &str) -> Self {
        self.criteria.contents_contains = search.to_string();
        self
    }

    /// Add exclude
    pub fn exclude(mut self, token_id: usize) -> Self {
        self.criteria.excludes.push(token_id.clone());
        self
    }

    /// Set excludes
    pub fn excludes(mut self, token_ids: &Vec<usize>) -> Self {
        self.criteria.excludes = token_ids.clone();
        self
    }

    /// Apply search criteria and iterate over matching tokens
    pub fn iter(mut self) -> TokenIter {
        let tokens = self.search();
        TokenIter::new(&tokens)
    }

    /// Search tag stack with criteria, return vector of tokens
    pub fn to_vec(mut self) -> Vec<Token> {
        self.search()
    }

    /// Search all tokens and apply criteria
    fn search(&mut self) -> Vec<Token> {
        // Initialize
        self.stack.set_parent_position(&self.criteria.parent_id);
        self.stack.set_excludes(&self.criteria.excludes);
        let crit = &self.criteria;
        let mut tokens: Vec<Token> = Vec::new();

        // Go through items
        while let Some(token) = self.stack.pull() {
            if ((!crit.tag.is_empty()) && token.tag() != crit.tag)
                || ((!crit.id.is_empty()) && !token.attr_equals("id", &crit.id))
                || ((!crit.class.is_empty()) && !token.attr_has_segment("class", &crit.class))
                || ((!crit.attr_key.is_empty())
                    && (!token.attr_equals(&crit.attr_key, &crit.attr_value)))
                || ((!crit.attr_contains_key.is_empty())
                    && (!token.attr_contains(&crit.attr_contains_key, &crit.attr_contains_value)))
                || ((!crit.contents.is_empty()) && token.contents() != crit.contents)
                || ((!crit.contents_contains.is_empty())
                    && !token.contents().contains(&crit.contents_contains))
            {
                continue;
            }

            let contents = self.stack.render_tag(&token.id());
            if !contents.is_empty() {
                let mtoken = self.stack.get_mut(&token.id()).unwrap();
                mtoken.set_contents(contents.as_str());
                tokens.push(mtoken.clone());
            } else {
                tokens.push(token);
            }
        }

        tokens
    }
}
