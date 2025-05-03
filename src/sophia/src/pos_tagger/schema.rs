use super::POSTag;
use crate::vocab::f8::f8;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Add;

/// A trait for types that can be used as scores in the POS tagger, requiring default, addition, and serialization capabilities.
pub trait Score: Default + Add + Serialize + for<'de> Deserialize<'de> {}

/// A part-of-speech tagger structure that maps tags to words and tracks tag, initial, before, and after scores.
#[derive(Default, Serialize, Deserialize)]
#[serde(
    bound = "T: Score, S: Default + Eq + PartialEq + Hash + Serialize + for<'a> Deserialize<'a>"
)]
pub struct POSTagger<T: Score, S> {
    pub tag2tag: POSTaggerLayer<T>,
    pub tag2word: HashMap<S, POSTaggerLayer<T>>,
    pub word2word: HashMap<u16, u16>,
}

/// A layer of the POS tagger, containing tag scores and initial, before, and after scoring structures.
#[derive(Serialize, Deserialize, Clone)]
#[serde(bound = "T: Score")]
pub struct POSTaggerLayer<T: Score> {
    pub tags: HashMap<POSTag, T>,
    pub initial: POSTaggerScores<T>,
    pub before: POSTaggerScores<T>,
    pub after: POSTaggerScores<T>,
}

/// Stores exact match trie and bigram scores for POS tagging.
#[derive(Default, Serialize, Deserialize, Clone)]
#[serde(bound = "T: Score")]
pub struct POSTaggerScores<T: Score> {
    pub exact_matches: POSTaggerExactMatchTrie,
    pub bigrams: Vec<POSTaggerBigramScores<T>>,
}

/// Stores bigram scores as a mapping from bigram identifiers to lists of tag-score pairs.
#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(bound = "T: Score")]
pub struct POSTaggerBigramScores<T: Score>(pub HashMap<u16, Vec<(POSTag, T)>>);

/// A trie structure for exact match POS tagging, mapping character sequences to tags.
#[derive(Default, Serialize, Deserialize, Clone)]
pub struct POSTaggerExactMatchTrie {
    pub tag: Option<POSTag>,
    pub children: HashMap<i8, Box<POSTaggerExactMatchTrie>>,
}

impl<T: Score, S: Default + Hash + Eq + Serialize + for<'de> Deserialize<'de>> POSTagger<T, S> {
    /// Creates a new POSTagger instance with default values.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Score for usize {}
impl Score for f32 {}
impl Score for f8 {}

impl<T: Score> POSTaggerScores<T> {
    pub fn new(size: usize) -> Self {
        Self {
            exact_matches: POSTaggerExactMatchTrie::default(),
            bigrams: (0..size)
                .map(|_| POSTaggerBigramScores::<T>::default())
                .collect::<Vec<POSTaggerBigramScores<T>>>(),
        }
    }
}

impl POSTaggerBigramScores<usize> {
    /// Increments the score for a given bigram and tag, adding a new entry if the tag is not present.
    pub fn incr(&mut self, bigram: u16, tag: POSTag) {
        let scores = self.0.entry(bigram).or_default();
        let index = match scores.iter().position(|score| score.0 == tag) {
            Some(r) => r,
            None => {
                scores.push((tag, 0));
                scores.len() - 1
            }
        };
        scores[index].1 += 1;
    }
}

impl<T: Score> Default for POSTaggerLayer<T> {
    fn default() -> Self {
        Self {
            tags: HashMap::new(),
            initial: POSTaggerScores::new(2),
            before: POSTaggerScores::new(4),
            after: POSTaggerScores::new(2),
        }
    }
}
