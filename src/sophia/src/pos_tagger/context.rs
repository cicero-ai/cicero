// Copyright 2025 Aquila Labs of Alberta, Canada <matt@cicero.sh>
// Licensed under the Functional Source License, Version 1.1 (FSL-1.1)
// See the full license at: https://cicero.sh/license.txt
// Distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND.

use std::fmt;
use serde::{Serialize, Deserialize};
use std::hash::{Hash, Hasher};
use crate::pos_tagger::POSTag;
use crate::pos_tagger::tagger::ResolvedScore;
use crate::vocab::{VocabDatabase, VocabMWE, Capitalization};
use crate::tokenizer::Token;

pub const MAX_TAGS_BEFORE: usize = 6;
pub const MAX_TAGS_AFTER: usize = 4;
pub const MAX_ANCHORS_BEFORE: usize = 4;
pub const MAX_ANCHORS_AFTER: usize = 2;

#[derive(Default)]
pub struct POSTaggerContext {
    tags: Vec<POSTag>,
    anchors: Vec<Anchor>,
    stoppers: Vec<usize>,
    pub words: Vec<ResolvedScore>,
    current_anchor: Anchor
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Anchor {
    pub position: usize,
    pub tag: POSTag,
    pub general_tag: POSTag,
    pub span: Vec<(SpanContent, Distance)>
}

#[derive(Default, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct AnchorIncludes {
    pub length: usize,
    pub is_exact_tag: bool,
    pub inc_distance: bool,
    pub inc_span: bool,
    pub inc_span_distance: bool
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum SpanContent {
    RB(String),
    JJ(String),
    DT,
    IN,
    IN_DT,
    MD,
    verb,   // non VB, VBG
    CC,
    CS,
    CA,
    PR,
    PRP,
    #[default]
    empty,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Distance {
    #[default]
    none,
    adjacent,
    short,
    medium,
    long,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
enum SentencePosition {
    #[default]
    first_word,
    last_word,
    beginning,
    middle,
    end,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Feature {
    pub position: usize,
    pub feature_type: FeatureType,
    capitalization: Capitalization,
    sentence_position: SentencePosition,
    pub tags_before: Vec<POSTag>,
    pub tags_after: Vec<POSTag>,
    pub anchors_before: Vec<Anchor>,
    pub anchors_after: Vec<Anchor>
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct DeterministicRule {
    pub tag: POSTag,
    pub feature: Feature,
    pub exceptions: Vec<(Feature, Option<POSTag>)>,
    pub siblings: Vec<Feature>
}

#[derive(Default, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum FeatureType {
    #[default]
    none,
    tag_before(usize),
    tag_after(usize),
    anchor_before(AnchorIncludes),
    anchor_after(AnchorIncludes),
    sentence_position,
    capitalization
}

impl POSTaggerContext {

    pub fn push(&mut self, token: &Token, vocab: &VocabDatabase) {

        let position = self.tags.len();
        self.tags.push(token.pos);
        if token.pos == POSTag::SS {
            self.stoppers.push(position);
        }

        // Check for continuation of current anchor (eg. compound noun)
        if self.is_continuation(token).is_some() {
            return;
        }

        // Check if new anchor found
        if self.is_new_anchor(position, token).is_some() {
            return;
        }

        // Get last span
        let last_span = match self.current_anchor.span.last() {
            Some(res) => res.0.clone(),
            None => SpanContent::empty
        };

        // Determine span content
        let span = match token {
            tk if tk.is_adverb() => SpanContent::RB(self.get_token_category(token, vocab)), // Note: Duplicate condition, should fix
            tk if tk.is_adjective() => SpanContent::JJ(self.get_token_category(token, vocab)),
            tk if tk.pos == POSTag::DT && last_span == SpanContent::IN => {
                self.current_anchor.span.last_mut().unwrap().0 = SpanContent::IN_DT;
                SpanContent::empty
            }
            tk if tk.pos == POSTag::DT => SpanContent::DT,
            tk if tk.pos == POSTag::IN => SpanContent::IN,
            tk if tk.pos == POSTag::MD => SpanContent::MD,
            tk if tk.is_verb() && tk.pos != POSTag::VB && tk.pos != POSTag::VBG => SpanContent::verb,
            tk if tk.pos == POSTag::CC => SpanContent::CC,
            tk if tk.pos == POSTag::CS => SpanContent::CS,
            tk if tk.pos == POSTag::CA => SpanContent::CA,
            tk if tk.pos == POSTag::PR => SpanContent::PR,
            tk if tk.pos == POSTag::PRP => SpanContent::PRP,
            _ => SpanContent::empty,
        };

        // Add to span
        if span != SpanContent::empty {
            let distance = position - self.current_anchor.position;
            self.current_anchor.span.push((span, Distance::from(distance)));
        }
    }

    /// Check whether or not this is a continuation of the current anchor (eg. compound noun, multi-punctuation, etc.)
    fn is_continuation(&self, token: &Token) -> Option<()> {

        if (token.pos.is_noun() && self.current_anchor.tag.is_noun()) 
            || ([POSTag::VB, POSTag::VBG].contains(&token.pos) && self.current_anchor.tag.is_verb()) 
            || (token.pos.is_punctuation() && self.current_anchor.tag.is_punctuation()) {
            return Some(());
        }

        None
    }

    /// Check if this is a new anchor
    fn is_new_anchor(&mut self, position: usize, token: &Token) -> Option<()> {

        if (token.is_noun() && !self.current_anchor.tag.is_noun()) 
            || ((token.pos == POSTag::VB || token.pos == POSTag::VBG) && !self.current_anchor.tag.is_verb()) 
            || ((token.pos == POSTag::SS || token.pos == POSTag::PUNC) && !self.current_anchor.tag.is_punctuation()) 
        {

            // Add current to anchors, if necessary
            if self.current_anchor.tag != POSTag::FW {
                self.anchors.push(self.current_anchor.clone());
                self.current_anchor = Anchor::new(position, token);
            }
            return Some(());
        }

        None
    }

    /// Get category name of a token
    fn get_token_category(&self, token: &Token, vocab: &VocabDatabase) -> String {

        for category_id in token.categories.iter() {
            let cat = vocab.categories.get(category_id).unwrap();
            let fqn = vocab.categories.get_fqn(&cat);
            if token.pos.is_adverb() && fqn[0] == "adverbs" {
                return fqn[1].to_string();
            } else if token.pos.is_adjective() && fqn[0] == "adjectives" {
                return fqn[1].to_string();
            }
        }

        "uncategorized".to_string()
    }

    /// Extract feature from the context for a specific word / position
    pub fn extract_feature(&self, mut position: usize, token: &Token) -> Feature {
        let mut res = Feature {
            position,
            capitalization: VocabMWE::classify_capitalization(&token.word),
            sentence_position: self.get_sentence_position(position),
            ..Default::default()
        };

        // Get anchor index
        let mut anchor_index = self.anchors.iter().position(|an| an.position > position).unwrap_or(0);

        // Before tags
        if position > 0 {
            let start = position.saturating_sub(MAX_TAGS_BEFORE);
            res.tags_before = self.tags[start..position].iter().rev().map(|tag| *tag).collect();
        }

        // Before anchors
        if anchor_index > 0 {
            let start = anchor_index.saturating_sub(MAX_ANCHORS_BEFORE);
            res.anchors_before = self.anchors[start..anchor_index].iter().rev().map(|anc| anc.clone()).collect();
        }


        // After tags
        position += 1;
        if position < self.tags.len() { 
            let end = (position + MAX_TAGS_AFTER).min(self.tags.len());
            res.tags_after = self.tags[position..end].to_vec();
        }

        // After anchors
        anchor_index += 1;
        if anchor_index < self.anchors.len() {
            let end = (anchor_index + MAX_ANCHORS_AFTER).min(self.anchors.len());
            res.anchors_after = self.anchors[anchor_index..end].to_vec();
        }

        res
    }

    /// Get sentence position

    fn get_sentence_position(&self, position: usize) -> SentencePosition {
        // Find the sentence boundaries for the given position
        let (sentence_start, sentence_end) = self.find_sentence_boundaries(position);
        
        // Calculate sentence length and relative position within sentence
        let sentence_length = sentence_end - sentence_start + 1;
        let relative_position = position - sentence_start;
        
        // Determine the position type
        match relative_position {
            0 => SentencePosition::first_word,
            pos if pos == sentence_length - 1 => SentencePosition::last_word,
            pos => {
                let quarter_length = sentence_length as f32 * 0.25;
                if (pos as f32) < quarter_length {
                    SentencePosition::beginning
                } else if (pos as f32) >= (sentence_length as f32 - quarter_length) {
                    SentencePosition::end
                } else {
                    SentencePosition::middle
                }
            }
        }
    }

    // Helper function to find sentence boundaries
    fn find_sentence_boundaries(&self, position: usize) -> (usize, usize) {
        // Find the start of the sentence
        let sentence_start = self.stoppers
            .iter()
            .rev()
            .find(|&&stopper_pos| stopper_pos < position)
            .map(|&stopper_pos| stopper_pos + 1)
            .unwrap_or(0);
        
        // Find the end of the sentence
        let sentence_end = self.stoppers
            .iter()
            .find(|&&stopper_pos| stopper_pos >= position)
            .copied()
            .unwrap_or(position); // If no stopper found after position, use position itself
        
        (sentence_start, sentence_end)
    }
}

impl Anchor {
    pub fn new(position: usize, token: &Token) -> Self {

            // Get general pos
        let general_tag = match token.pos {
            t if t.is_noun() => POSTag::NN,
            t if t.is_verb() => POSTag::VB,
            t if t.is_punctuation() => POSTag::PUNC,
            _ => POSTag::FW
        };

        Self {
            position,
            tag: token.pos,
            general_tag,
            ..Default::default()
        }
    }

    pub fn to_debug(&self, specs: &AnchorIncludes) -> String {

        let mut res = if specs.is_exact_tag {
            format!("exact tag {}", self.tag)
        } else {
            format!("general tag {}", self.general_tag)
        };

        if specs.inc_distance {
            res = format!("{} d", res);
        }
        if !specs.inc_span { return res; }

        let span_res = self.span.iter().map(|(span, dist)| {
            if specs.inc_span_distance { 
                format!("{:?} d{:?}", span, dist) 
            } else { 
                format!("{:?}", span) 
            }
        }).collect::<Vec<String>>().join(" ").to_string();

        format!("{} span {}", res, span_res)
    }
}

impl From<usize> for Distance {
    fn from(value: usize) -> Self {
        match value {
            val if val <= 1 => Self::adjacent,
            val if val <= 3 => Self::short,
            val if val <= 6 => Self::medium,
            _ => Self::long
        }
    }
}

impl AnchorIncludes {
    pub fn new(length: usize, is_exact_tag: bool, inc_distance: bool, inc_span: bool, inc_span_distance: bool) -> Self {
        Self { length, is_exact_tag, inc_distance, inc_span, inc_span_distance }
    }

    pub fn all(length: usize, direction: &str) -> Vec<FeatureType> {
        let mut res = vec![];
        let x = length;

        res.push(Self::new(x, false, false, false, false));
        res.push(Self::new(x, false, true, false, false));
        res.push(Self::new(x, false, true, true, false));
        res.push(Self::new(x, false, true, true, true));

        res.push(Self::new(x, true, false, false, false));
        res.push(Self::new(x, true, true, false, false));
        res.push(Self::new(x, true, true, true, false));
        res.push(Self::new(x, true, true, true, true));

        // Return
        res.iter().map(|specs| {
            if direction == "after" { FeatureType::anchor_after(*specs) } else { FeatureType::anchor_before(*specs) }
        }).collect::<Vec<FeatureType>>()
    }
}

impl Hash for Feature {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.feature_type.hash(state);

        let mut specs = AnchorIncludes::default();
        let mut anchor_vec = &self.anchors_before;

        match &self.feature_type {
            FeatureType::tag_before(num) => {
                let end = (*num as usize).min(self.tags_before.len());
                self.tags_before[0..end].to_vec().hash(state);
            },
            FeatureType::tag_after(num) => {
                let end = (*num as usize).min(self.tags_after.len());
                self.tags_after[0..end].to_vec().hash(state);
            },
            FeatureType::anchor_before(includes) => specs = includes.clone(),
            FeatureType::anchor_after(includes) => {
                specs = includes.clone();
                anchor_vec = &self.anchors_after;
            },
            FeatureType::sentence_position => self.sentence_position.hash(state),
            FeatureType::capitalization => self.capitalization.hash(state),
            _ => unreachable!()
        };
        if specs.length == 0 { return; }

        // Hash anchors
        for (x, anchor) in anchor_vec.iter().enumerate() {
            if x >= specs.length { break; }

            if specs.is_exact_tag {
                anchor.tag.hash(state);
            } else {
                anchor.general_tag.hash(state);
            }

            if specs.inc_distance {
                let distance: usize = if anchor.position > self.position { anchor.position - self.position } else { self.position - anchor.position };
                Distance::from(distance).hash(state);
            }
            if !specs.inc_span { continue; }

            for (span, dist) in anchor.span.iter() {
                span.hash(state);
                if specs.inc_span_distance {
                    dist.hash(state);
                }
            }
        }
    }
}

impl PartialEq for Feature {
    fn eq(&self, other: &Self) -> bool {
        // First, check if feature_type matches
        if self.feature_type != other.feature_type {
            return false;
        }

        // Compare fields based on feature_type, mirroring the Hash implementation
        match &self.feature_type {
            FeatureType::tag_before(num) => {
                let end = (*num as usize).min(self.tags_before.len()).min(other.tags_before.len());
                self.tags_before[0..end] == other.tags_before[0..end]
            }
            FeatureType::tag_after(num) => {
                let end = (*num as usize).min(self.tags_after.len()).min(other.tags_after.len());
                self.tags_after[0..end] == other.tags_after[0..end]
            }
            FeatureType::anchor_before(includes) => {
                self.compare_anchors(&self.anchors_before, &other.anchors_before, self.position, includes)
            }
            FeatureType::anchor_after(includes) => {
                self.compare_anchors(&self.anchors_after, &other.anchors_after, self.position, includes)
            }
            FeatureType::sentence_position => self.sentence_position == other.sentence_position,
            FeatureType::capitalization => self.capitalization == other.capitalization,
            _ => unreachable!()
        }
    }
}

impl Feature {
    pub fn with_type(mut self, ftype: FeatureType) -> Self {
        self.feature_type = ftype;
        self
    }

    fn compare_anchors(&self, anchors1: &[Anchor], anchors2: &[Anchor], position: usize, specs: &AnchorIncludes) -> bool {
        // Compare up to specs.length anchors
        let len = specs.length.min(anchors1.len()).min(anchors2.len());
        for i in 0..len {
            let anchor1 = &anchors1[i];
            let anchor2 = &anchors2[i];

            // Compare tag
            if specs.is_exact_tag && anchor1.tag != anchor2.tag {
                return false;
            } else if (!specs.is_exact_tag) && anchor1.general_tag != anchor2.general_tag {
                return false;
            }

            // Compare distance if inc_distance is true
            if specs.inc_distance {
                let distance1: usize = if anchor1.position > position { anchor1.position - position } else { position - anchor1.position };
                let distance2: usize = if anchor2.position > position { anchor2.position - position } else { position - anchor2.position };
                if Distance::from(distance1) != Distance::from(distance2) {
                    return false;
                }
            }

            // Compare span if inc_span is true
            if specs.inc_span {
                if anchor1.span.len() != anchor2.span.len() {
                    return false;
                }
                for ((span1, dist1), (span2, dist2)) in anchor1.span.iter().zip(anchor2.span.iter()) {
                    if span1 != span2 {
                        return false;
                    }
                    // Compare span distance if inc_span_distance is true
                    if specs.inc_span_distance && dist1 != dist2 {
                        return false;
                    }
                }
            }
        }
        true
    }
}

impl Eq for Feature {}

impl fmt::Debug for Feature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        if let FeatureType::tag_before(num) = self.feature_type {
            let tags = self.tags_before.iter().map(|tag| tag.to_string()).collect::<Vec<String>>().join(" ").to_string();
            write!(f, "{} tags before -- {}", num, tags)
        } else if let FeatureType::tag_after(num) = self.feature_type {
            let tags = self.tags_after.iter().map(|tag| tag.to_string()).collect::<Vec<String>>().join(" ").to_string();
            write!(f, "{} tags after -- {}", num, tags)
        } else if let FeatureType::anchor_before(specs) = self.feature_type {
            let mut res = Vec::new();
            for (x, a) in self.anchors_before.iter().enumerate() {
                if x >= specs.length { break; }
                res.push(a.to_debug(&specs));
            }
            write!(f, "{} anchors before: {}", specs.length, res.join(" | ").to_string())
        } else if let FeatureType::anchor_after(specs) = self.feature_type {
            let mut res = Vec::new();
            for (x, a) in self.anchors_after.iter().enumerate() {
                if x >= specs.length { break; }
                res.push(a.to_debug(&specs));
            }
            write!(f, "{} anchors after: {}", specs.length, res.join(" | ").to_string())

        } else if FeatureType::sentence_position == self.feature_type {
            write!(f, "sentence pos: {:?}", self.sentence_position)

        } else {
            write!(f, "Unknown")
        }

    }
}

impl FeatureType {
    pub fn to_u8(&self) -> u8 {
        match self {
            Self::none => 0,
            Self::sentence_position => 1,
            Self::capitalization => 2,
            Self::tag_before(_num) => 3,
            Self::tag_after(_num) => 4,
            Self::anchor_before(_specs) => 5,
            Self::anchor_after(_specs) => 6
        }
    }
}


