// Copyright 2025 Aquila Labs of Alberta, Canada <matt@cicero.sh>
// Licensed under the PolyForm Noncommercial License 1.0.0
// Commercial use requires a separate license: https://cicero.sh/sophia/
// License text: https://polyformproject.org/licenses/noncommercial/1.0.0/
// Distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND.

use super::VocabDatabase;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::ops::Range;

/// A database for storing vocabulary categories, including nouns, verbs, adverbs, adjectives, and named entity recognition (NER) indices.
#[derive(Default, Serialize, Deserialize, Clone)]
pub struct VocabCategoryDatabase {
    pub counter: i16,
    pub nodes: HashMap<i16, VocabCategory>,
    pub nouns: VocabCategoryIndex,
    pub verbs: VocabCategoryIndex,
    pub adverbs: VocabCategoryIndex,
    pub adjectives: VocabCategoryIndex,
    pub ner: VocabCategoryIndex,
}

/// A trie-like index for vocabulary categories, mapping paths to category indices and their children.
#[derive(Serialize, Deserialize, Clone)]
pub struct VocabCategoryIndex {
    pub index: i16,
    pub children: IndexMap<String, Box<VocabCategoryIndex>>,
}

/// Represents a single category with its fully qualified name (FQN), depth, name, and child categories.
#[derive(Serialize, Deserialize, Clone)]
pub struct VocabCategory {
    pub fqn: Vec<i16>,
    pub depth: i8,
    pub name: String,
    pub children: IndexMap<String, i16>,
    #[serde(skip)]
    pub pos: String,
    #[serde(skip)]
    pub words: Vec<i32>,
}

impl VocabCategoryDatabase {
    /// Get category by path name
    pub fn get_category_by_path(&self, path: &str, vocab: &VocabDatabase) -> Option<VocabCategory> {
        // Split path
        let parts: Vec<&str> = path.split("/").collect::<Vec<&str>>();
        if parts.len() < 2 {
            return None;
        }
        let remaining_path = parts[1..].join("/").to_string();

        // Get index
        let cat_index = match parts[0] {
            "nouns" => self.nouns.index_by_path(&remaining_path)?,
            "verbs" => self.verbs.index_by_path(&remaining_path)?,
            "adverbs" => self.adverbs.index_by_path(&remaining_path)?,
            "adjectives" => self.adjectives.index_by_path(&remaining_path)?,
            "ner" => self.ner.index_by_path(&remaining_path)?,
            _ => return None,
        };
        let index = cat_index.index;

        // Get category
        let mut cat: VocabCategory = self.nodes.get(&index)?.clone();
        cat.pos = parts[0].to_string();
        cat.words = vocab
            .words
            .id2token
            .iter()
            .filter(|(_, token)| token.categories.contains(&index))
            .map(|(id, _)| *id)
            .collect();

        Some(cat)
    }
}

impl VocabCategoryIndex {
    /// Creates a new VocabCategoryIndex with default values.
    pub fn new() -> Self {
        Self {
            index: 0,
            children: IndexMap::new(),
        }
    }

    /// Inserts a category path into the index, assigning the given index and returning it.
    pub fn insert(&mut self, path: &str, index: i16) -> i16 {
        let mut current = self;
        for word in path.split("/").collect::<Vec<&str>>().iter() {
            current = current
                .children
                .entry(word.to_lowercase().to_string())
                .or_insert(Box::new(VocabCategoryIndex::new()));
        }

        current.index = index;
        index
    }

    /// Retrieves the category index for a given path, if it exists.
    pub fn by_path(&self, path: &str) -> Option<i16> {
        if let Some(cat) = self.index_by_path(path) {
            return Some(cat.index);
        }
        None
    }

    /// Retrieves the VocabCategoryIndex object for a given path, if it exists.
    pub fn index_by_path(&self, path: &str) -> Option<VocabCategoryIndex> {
        let mut current = self;
        for word in path.to_lowercase().split("/").collect::<Vec<&str>>().iter() {
            match current.children.get(&word.to_string()) {
                Some(next) => current = next.as_ref(),
                None => return None,
            }
        }
        Some(current.clone())
    }

    /// Returns the range of category IDs for a path, including its children, if the path exists.
    pub fn path2range(&self, path: &str) -> Option<Range<i16>> {
        if let Some(r) = self.index_by_path(path) {
            return Some(r.index..(r.index + (r.count_children() + 1) as i16));
        }

        None
    }

    /// Counts the total number of children under this index, including nested children.
    pub fn count_children(&self) -> usize {
        let mut count = self.children.len();

        for (_, child) in self.children.iter() {
            count += child.count_children();
        }
        count
    }
}

impl VocabCategoryDatabase {
    /// Retrieves a category by its ID, if it exists.
    pub fn get(&self, category_id: &i16) -> Option<VocabCategory> {
        if let Some(cat) = self.nodes.get(&category_id.clone()) {
            return Some(cat.clone());
        }
        None
    }

    /// Retrieves a noun category by its path, if it exists.
    pub fn nouns_by_path(&self, path: &str) -> Option<VocabCategory> {
        match self.nouns.by_path(path) {
            Some(r) => self.get(&r),
            None => None,
        }
    }

    /// Returns the number of children for a given category ID.
    pub fn get_children_count(&self, category_id: &i16) -> usize {
        let node = self.nodes.get(&category_id.clone()).unwrap();
        node.children.len()
    }

    /// Retrieves the fully qualified names of a category's parent categories.
    pub fn get_fqn(&self, category: &VocabCategory) -> Vec<String> {

        let mut names: Vec<String> = Vec::new();
        for parent_id in category.fqn.iter() {
            let parent_name: String = match self.nodes.get(parent_id) {
                Some(r) => r.name.to_string(),
                None => String::from("Uknown"),
            };
            names.push(parent_name.to_string());
        }

        names
    }
}

impl VocabCategory {}

impl fmt::Display for VocabCategory {
    /// Formats the VocabCategory for display, showing its name.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Debug for VocabCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fqn = self.fqn.iter().map(|id| id.to_string()).collect::<Vec<String>>();
        write!(f, "{} -> {}", fqn.join("/"), self.name)
    }
}

impl Default for VocabCategoryIndex {
    fn default() -> VocabCategoryIndex {
        VocabCategoryIndex {
            index: 0,
            children: IndexMap::new(),
        }
    }
}
