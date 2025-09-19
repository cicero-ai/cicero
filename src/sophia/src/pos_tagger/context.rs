// Copyright 2025 Aquila Labs of Alberta, Canada <matt@cicero.sh>
// Licensed under the PolyForm Noncommercial License 1.0.0
// Commercial use requires a separate license: https://cicero.sh/sophia/
// License text: https://polyformproject.org/licenses/noncommercial/1.0.0/
// Distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND.

use std::hash::Hash;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use super::{POSTag, TokenKey};
use crate::tokenizer::Token;
use crate::vocab::{PronounCategory, PronounPerson, PronounNumber};
use crate::Error;

pub const SIBLING_TAGS_BEFORE: usize = 8;
pub const SIBLING_TAGS_AFTER: usize = 4;

pub static MODAL_VERBS: &[&str] = &["can", "could", "may", "might", "must", "shall", "should", "will", "would"];
pub static PASSIVE_INDICATORS: &[&str] = &["am", "are", "be", "been", "being", "is", "was", "were", "get", "gets", "getting", "got", "gotten"];
pub static AUXILLARY_VERBS: &[&str] = &["am", "are", "be", "been", "being", "can", "could", "did", "do", "does", "doing", "had", "has", "have", "having", "is", "may", "might", "must", "shall", "should", "was", "were", "will", "would"];
pub static PERFECT_TENSE_INDICATORS: &[&str] = &["have", "has", "had", "having"];
pub static TEMPORAL_ADVERBS: &[&str] = &["now", "then", "today", "yesterday", "tomorrow", "always", "often", "sometimes", "never", "rarely", "usually", "frequently", "seldom", "ever", "already", "yet", "still", "just", "soon", "recently", "lately", "before", "after", "first", "next", "last", "finally", "forever", "briefly"];
pub static COMMON_ADVERBS: &[&str] = &[
    "quickly", "slowly", "carefully", "easily", "quietly", "loudly", "clearly", "closely", "simply", "suddenly",
    "always", "often", "usually", "sometimes", "rarely", "never", "frequently", "occasionally",
    "very", "too", "quite", "almost", "nearly", "hardly", "barely", "enough",
    "now", "then", "today", "yesterday", "tomorrow", "soon", "later", "already", "still", "yet",
    "even", "only", "just", "also", "however", "therefore", "thus", "otherwise", "rather", "indeed"
];

#[derive(Clone, Serialize, Deserialize)]
#[serde(bound = "S: Default + Clone + Eq + PartialEq + Hash + Serialize + for<'a> Deserialize<'a>")]
pub struct POSContext<S>(pub Vec<Vec<POSFeatureToken<S>>>);

pub struct POSContextIter<'a, S> {
    context: &'a POSContext<S>,
    indices: Vec<usize>,
    outer_index: usize,
    inner_index: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(bound = "S: Default + Clone + Eq + PartialEq + Hash + Serialize + for<'a> Deserialize<'a>")]
pub struct POSConjunction<S> {
    pub tags: HashMap<POSTag, f32>,
    pub weight: f32,
    pub mi_score: f32,
    pub siblings: Vec<POSFeature<S>>
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(bound = "S: Default + Clone + Eq + PartialEq + Hash + Serialize + for<'a> Deserialize<'a>")]
pub struct POSFeature<S> {
    pub feature_token: POSFeatureToken<S>,
    pub offset: i8,
    pub noise_profile: u8
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(bound = "S: Default + Clone + Eq + PartialEq + Hash + Serialize + for<'a> Deserialize<'a>")]
pub enum POSFeatureToken<S> {
    tag(POSTag),
    tag_group(POSTagGroup),
    word_group(POSWordGroup),
    word(S),
    suffix(POSSuffix),
    pronoun_category(PronounCategory),
    pronoun_person(PronounPerson),
    pronoun_number(PronounNumber)
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum POSTagGroup {
    noun,
    verb,
    base_verb,
    current_verb,
    past_verb,
    adverb,
    adjective,
    pronoun
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum POSWordGroup {
    modal_verb,
    passive_indicator,
    auxillary_verb,
    perfect_tense_indicator,
    temporal_adverb,
    common_adverb
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum POSSuffix {
    ed,
    ing,
    ly,
    day,
    s,
    en,
    er,
    est,
    t,
    tion,
    ion,
    al,
    ous,
    ful,
    less,
    able,
    ible,
    ive,
    ness,
    ment,
    ity,
    ty,
    ance,
    ence,
    age,
    ship,
    hood,
    ward,
    wise
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum POSPrefix {
    un,
    re,
    r#in,
    dis,
    en,
    em,
    non,
    pre,
    pro,
    anti,
    de,
    mis,
    over,
    under,
    counter,
    mal,
    sub,
    semi,
    multi,
    mini,
    micro,
    mega,
    inter,
    intra,
    trans,
    extra,
    intro,
    retro,
    circum,
    post,
    fore,
    ante,
    uni,
    bi,
    tri,
    quad,
    poly,
    mono,
    pseudo,
    quasi,
    auto,
    co,
    com,
    con,
    ex
}

impl<S> Default for POSContext<S>
where S: Default + Clone + Eq + PartialEq + Hash + Serialize + for<'a> Deserialize<'a>, Token: TokenKey<S>
 {
    fn default() -> Self {
        Self::new()
    }
}

impl<S> POSContext<S> 
    where S: Default + Clone + Eq + PartialEq + Hash + Serialize + for<'a> Deserialize<'a>, Token: TokenKey<S>
{

        pub fn new() -> Self {
        Self(vec![vec![]; SIBLING_TAGS_BEFORE + SIBLING_TAGS_AFTER + 1])
    }

    /// Build context of ambiguous token from position within a vector of tokens
    pub fn from_tokens(position: usize, tokens: &[Token]) -> Self {
        let mut context = Self::new();

        // Before siblings
        for offset in 1..=SIBLING_TAGS_BEFORE {
            if position < offset || tokens[position - offset].pos == POSTag::SS {
                break;
            }

            context.0[SIBLING_TAGS_BEFORE - offset] = POSFeatureToken::build_from_token(&tokens[position-offset]);
        }

        // After siblings
        for offset in 1..=SIBLING_TAGS_AFTER {
            if (position + offset) >= tokens.len() || tokens[position + offset].pos == POSTag::SS {
                break;
            }

            context.0[SIBLING_TAGS_BEFORE + offset] = POSFeatureToken::build_from_token(&tokens[position + offset]);
        }

        context
    }

    pub fn iter_ft(&self) -> POSContextIter<'_, S> {

        // Get indices
        let start = SIBLING_TAGS_BEFORE + 1;
        let mut indices: Vec<usize> = (0..SIBLING_TAGS_BEFORE).rev().collect();
        indices.extend(&(start..(start + SIBLING_TAGS_AFTER)).collect::<Vec<usize>>());

        POSContextIter {
            context: self,
            indices,
            outer_index: 0,
            inner_index: 0
        }
    }

}

impl<S> POSFeature<S> 
    where S: Default + Clone + Eq + PartialEq + Hash + Serialize + for<'a> Deserialize<'a>, Token: TokenKey<S>
{
    pub fn new(feature_token: POSFeatureToken<S>, offset: i8, noise_profile: u8) -> POSFeature<S> {
        Self {
            feature_token,
            offset,
            noise_profile
        }
    }

    pub fn get_score(&self) -> f32 {
        let base_weight = self.feature_token.get_base_weight();

        // Get distance weight
        let distance_base = if self.offset < 0 {
            SIBLING_TAGS_BEFORE as i8 - self.offset.abs() 
        } else {
            SIBLING_TAGS_AFTER as i8 - self.offset
        };

        let distance_weight = if self.offset < 0 {
            ((distance_base * 15) as f32 / 100.0) + 1.0
        } else {
            ((distance_base * 25) as f32 / 100.0) + 1.0
        };

        base_weight * distance_weight
    }

    pub fn get_index(&self) -> usize {
        if self.offset < 0 {
            SIBLING_TAGS_BEFORE - self.offset.unsigned_abs() as usize
        } else {
            SIBLING_TAGS_BEFORE + self.offset as usize
        }
    }
}

impl<S> POSFeatureToken<S> 
    where S: Default + Clone + Eq + PartialEq + Hash + Serialize + for<'a> Deserialize<'a>, Token: TokenKey<S>
{
    // Build all possible feature tokens from a Token struct
    pub fn build_from_token(token: &Token) -> Vec<Self> {
        let mut res = vec![
            Self::tag(token.pos), 
            Self::word(token.get_key())
        ];

        // Standard tag group
        if let Ok(tag_group) = POSTagGroup::try_from(token) {
            res.push(Self::tag_group(tag_group));
        }

        // Granular verb based tag groups
        if token.pos == POSTag::VB {
            res.push(Self::tag_group(POSTagGroup::base_verb));
        } else if [POSTag::VBG, POSTag::VBZ].contains(&token.pos) {
            res.push(Self::tag_group(POSTagGroup::current_verb));
        } else if [POSTag::VBN, POSTag::VBD, POSTag::VBP].contains(&token.pos) {
            res.push(Self::tag_group(POSTagGroup::past_verb));
        }

        // Try the word groups
        if MODAL_VERBS.contains(&token.word.to_lowercase().as_str()) {
            res.push(Self::word_group(POSWordGroup::modal_verb));
        }
        if PASSIVE_INDICATORS.contains(&token.word.to_lowercase().as_str()) {
            res.push(Self::word_group(POSWordGroup::passive_indicator));
        }
        if AUXILLARY_VERBS.contains(&token.word.to_lowercase().as_str()) {
            res.push(Self::word_group(POSWordGroup::auxillary_verb));
        }
        if PERFECT_TENSE_INDICATORS.contains(&token.word.to_lowercase().as_str()) {
            res.push(Self::word_group(POSWordGroup::perfect_tense_indicator));
        }
        if TEMPORAL_ADVERBS.contains(&token.word.to_lowercase().as_str()) {
            res.push(Self::word_group(POSWordGroup::temporal_adverb));
        }
        if COMMON_ADVERBS.contains(&token.word.to_lowercase().as_str()) {
            res.push(Self::word_group(POSWordGroup::common_adverb));
        }

        if let Ok(suffix) = POSSuffix::try_from(token) {
            res.push(Self::suffix(suffix));
        }

        if let Some(pronoun) = &token.pronoun {
            res.push(Self::pronoun_category(pronoun.category.clone()));
            res.push(Self::pronoun_person(pronoun.person.clone()));
            res.push(Self::pronoun_number(pronoun.number.clone()));
        }

        res
    }

    /// Whether or not the feature can be used as a primary / anchor feature
    pub fn is_primary(&self) -> bool {
        matches!(self, Self::tag(_) | Self::tag_group(_) | Self::word_group(_) | Self::word(_))
    }

    // Convert token into a feature
    pub fn to_feature(&self, offset: usize, noise_profile: u8) -> POSFeature<S> {
        let f_offset = if offset >= SIBLING_TAGS_BEFORE { (offset - SIBLING_TAGS_BEFORE) as i8 } else { 0_i8 - offset as i8 };
        POSFeature::new(self.clone(), f_offset, noise_profile)
    }

    /// Check if token contains this feature
    pub fn contains_token(&self, token: &Token) -> bool {
        match self {
            Self::tag(tag) => token.pos == *tag,
            Self::tag_group(tag_group) => tag_group.is_group(token),
            Self::word_group(word_group) => word_group.is_group(token),
            Self::word(word) => token.get_key() == *word,
            Self::suffix(suffix) => suffix.token_has(token),
            Self::pronoun_category(category) => token.pronoun.as_ref().unwrap().category == *category,
            Self::pronoun_person(person) => token.pronoun.as_ref().unwrap().person == *person,
            Self::pronoun_number(number) => token.pronoun.as_ref().unwrap().number == *number
        }
    }

    /// Get base weight of feature token
    pub fn get_base_weight(&self) -> f32 {
        match &self {
            POSFeatureToken::word(_) => 1.8,
            POSFeatureToken::tag(_) => 1.6,
            POSFeatureToken::word_group(_) => 1.4,
            POSFeatureToken::tag_group(_) => 1.2,
            _ => 1.0
        }
    }
}

impl POSTagGroup {
    /// Check whether or not token exists to group
    pub fn is_group(&self, token: &Token) -> bool {
        (*self == Self::noun && token.is_noun()) ||
            (*self == Self::pronoun && token.is_pronoun()) || 
            (*self == Self::verb && token.is_verb()) || 
            (*self == Self::base_verb && token.pos == POSTag::VB) || 
            (*self == Self::current_verb && [POSTag::VBG, POSTag::VBZ].contains(&token.pos)) || 
            (*self == Self::past_verb && [POSTag::VBD, POSTag::VBP, POSTag::VBN].contains(&token.pos)) || 
            (*self == Self::adverb && token.is_adverb()) || (*self == Self::adjective && token.is_adjective())
    }
}

impl POSWordGroup {
    /// Check whether or not token exists to group
    pub fn is_group(&self, token: &Token) -> bool {
        let lowered = token.word.to_lowercase();
        let word = lowered.as_str();

        (*self == Self::modal_verb && MODAL_VERBS.contains(&word)) || 
            (*self == Self::auxillary_verb && AUXILLARY_VERBS.contains(&word)) || 
            (*self == Self::passive_indicator && PASSIVE_INDICATORS.contains(&word)) || 
            (*self == Self::perfect_tense_indicator && PERFECT_TENSE_INDICATORS.contains(&word)) || 
            (*self == Self::temporal_adverb && TEMPORAL_ADVERBS.contains(&word)) || (*self == Self::common_adverb && COMMON_ADVERBS.contains(&word))
    }
}

impl POSSuffix {
    /// Check whether or not token has the suffix
    pub fn token_has(&self, token: &Token) -> bool {
        let word = token.word.to_lowercase();
        let suffix = format!("{:?}", self).to_lowercase();
        word.ends_with(&suffix)
    }
}

impl TryFrom<&Token> for POSTagGroup {
    type Error = Error;

    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        let res = match token {
            t if t.is_noun() => Self::noun,
            t if t.is_verb() => Self::verb,
            t if t.is_adverb() => Self::adverb,
            t if t.is_adjective() => Self::adjective,
            t if t.is_pronoun() => Self::pronoun,
            _ => return Err(Error::Generic("No tag group available.".to_string()))
        };

        Ok(res)
    }
}

impl TryFrom<&Token> for POSWordGroup {
    type Error = Error;

    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        let lowered = token.word.to_lowercase();
        let word = lowered.as_str();

        let res = match token {
            _ if MODAL_VERBS.contains(&word) => Self::modal_verb,
            _ if PASSIVE_INDICATORS.contains(&word) => Self::passive_indicator,
            _ if AUXILLARY_VERBS.contains(&word) => Self::auxillary_verb,
            _ if PERFECT_TENSE_INDICATORS.contains(&word) => Self::perfect_tense_indicator,
            _ if TEMPORAL_ADVERBS.contains(&word) => Self::temporal_adverb,
            _ if COMMON_ADVERBS.contains(&word) => Self::common_adverb,
            _ => return Err(Error::Generic("No word group available.".to_string()))
        };

        Ok(res)
    }
}

impl TryFrom<&Token> for POSSuffix {
    type Error = Error;

    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        let lowered = token.word.to_lowercase();
        let word = lowered.as_str();

        // Check each suffix in order of length (longer first to avoid partial matches)
        // For example, "tion" should match before "ion"
        let suffixes = [
            (POSSuffix::tion, "tion"),
            (POSSuffix::able, "able"),
            (POSSuffix::ible, "ible"),
            (POSSuffix::ance, "ance"),
            (POSSuffix::ence, "ence"),
            (POSSuffix::ment, "ment"),
            (POSSuffix::ness, "ness"),
            (POSSuffix::ship, "ship"),
            (POSSuffix::hood, "hood"),
            (POSSuffix::ward, "ward"),
            (POSSuffix::wise, "wise"),
            (POSSuffix::ing, "ing"),
            (POSSuffix::est, "est"),
            (POSSuffix::ous, "ous"),
            (POSSuffix::day, "day"),
            (POSSuffix::ful, "ful"),
            (POSSuffix::less, "less"),
            (POSSuffix::ive, "ive"),
            (POSSuffix::ion, "ion"),
            (POSSuffix::ity, "ity"),
            (POSSuffix::age, "age"),
            (POSSuffix::ed, "ed"),
            (POSSuffix::en, "en"),
            (POSSuffix::er, "er"),
            (POSSuffix::ly, "ly"),
            (POSSuffix::al, "al"),
            (POSSuffix::ty, "ty"),
            (POSSuffix::s, "s"),
            (POSSuffix::t, "t"),
        ];

        for (suffix_enum, suffix_str) in suffixes.iter() {
            if word.ends_with(suffix_str) {
                return Ok(*suffix_enum);
            }
        }
        
        Err(Error::Generic("No suffix available".to_string()))
    }
}

impl POSPrefix {
    /// Check whether or not token has the prefix
    pub fn token_has(&self, token: &Token) -> bool {
        let word = token.word.to_lowercase();
        let prefix = format!("{:?}", self).to_lowercase();
        word.starts_with(&prefix)
    }
}

impl TryFrom<&Token> for POSPrefix {
    type Error = Error;

    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        let lowered = token.word.to_lowercase();
        let word = lowered.as_str();

        // Check each prefix in order of length (longer first to avoid partial matches)
        // For example, "counter" should match before "co"
        let prefixes = [
            (POSPrefix::counter, "counter"),
            (POSPrefix::inter, "inter"),
            (POSPrefix::intra, "intra"),
            (POSPrefix::trans, "trans"),
            (POSPrefix::extra, "extra"),
            (POSPrefix::intro, "intro"),
            (POSPrefix::retro, "retro"),
            (POSPrefix::circum, "circum"),
            (POSPrefix::multi, "multi"),
            (POSPrefix::micro, "micro"),
            (POSPrefix::pseudo, "pseudo"),
            (POSPrefix::quasi, "quasi"),
            (POSPrefix::under, "under"),
            (POSPrefix::over, "over"),
            (POSPrefix::anti, "anti"),
            (POSPrefix::fore, "fore"),
            (POSPrefix::ante, "ante"),
            (POSPrefix::semi, "semi"),
            (POSPrefix::mini, "mini"),
            (POSPrefix::mega, "mega"),
            (POSPrefix::post, "post"),
            (POSPrefix::auto, "auto"),
            (POSPrefix::un, "un"),
            (POSPrefix::re, "re"),
            (POSPrefix::r#in, "in"),
            (POSPrefix::dis, "dis"),
            (POSPrefix::en, "en"),
            (POSPrefix::em, "em"),
            (POSPrefix::non, "non"),
            (POSPrefix::pre, "pre"),
            (POSPrefix::pro, "pro"),
            (POSPrefix::de, "de"),
            (POSPrefix::mis, "mis"),
            (POSPrefix::mal, "mal"),
            (POSPrefix::sub, "sub"),
            (POSPrefix::uni, "uni"),
            (POSPrefix::bi, "bi"),
            (POSPrefix::tri, "tri"),
            (POSPrefix::quad, "quad"),
            (POSPrefix::poly, "poly"),
            (POSPrefix::mono, "mono"),
            (POSPrefix::co, "co"),
            (POSPrefix::com, "com"),
            (POSPrefix::con, "con"),
            (POSPrefix::ex, "ex"),
        ];

        for (prefix_enum, prefix_str) in prefixes.iter() {
            if word.starts_with(prefix_str) {
                return Ok(*prefix_enum);
            }
        }

        Err(Error::Generic("No prefix available".to_string()))
    }
}

impl<'a, S> Iterator for POSContextIter<'a, S> 
    where S: Default + Clone + Eq + PartialEq + Hash + Serialize + for<'b> Deserialize<'b>, Token: TokenKey<S>
{
    type Item = POSFeature<S>;

    fn next(&mut self) -> Option<Self::Item> {

        //  Update pointer as necessary
        if self.inner_index >= self.context.0[self.indices[self.outer_index]].len() {
            self.inner_index = 0;
            self.outer_index += 1;
            if self.outer_index >= self.indices.len() { return None; }

            while self.context.0[self.indices[self.outer_index]].is_empty() {
                self.outer_index += 1;
                if self.outer_index >= self.indices.len() {
                    return None;
                }
            }
        }

        // Check if we're exhausted (ie. only single word being tokenized)
        if self.context.0[self.indices[self.outer_index]].is_empty() {
            return None;
        }

        // GEt next item
        let f_token = self.context.0[self.indices[self.outer_index]][self.inner_index].clone();
        self.inner_index += 1;

        Some(f_token.to_feature(self.indices[self.outer_index], 0))
    }
}


