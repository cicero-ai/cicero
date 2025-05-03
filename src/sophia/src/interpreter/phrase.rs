use crate::tokenizer::Token;
use crate::vocab::{PhraseIntent, VocabDatabase};
use std::fmt;
use std::ops::Range;

/// Represents a phrase with a range of tokens, split token, nouns, verbs, tense, person, and classification.
#[derive(Default, Clone)]
pub struct Phrase {
    pub range: Range<usize>,
    pub split_token: Option<usize>,
    pub nouns: Vec<Noun>,
    pub verbs: Vec<Verb>,
    pub tense: PhraseTense,
    pub person: PhrasePerson,
    pub classification: PhraseClassification,
    pub intent: (PhraseIntent, f32),
}

/// Represents a noun with a head token, compound elements, modifiers, siblings, owner, and associated linguistic elements.
#[derive(Default, Clone)]
pub struct Noun {
    pub head: usize,
    pub compound_elements: Vec<usize>,
    pub modifiers: Vec<NounModifier>,
    pub siblings: Vec<NounSibling>,
    pub owner: NounOwner,
    pub prepositions: Vec<usize>,
    pub determiners: Vec<usize>,
    pub adjectives: Vec<Adjective>,
    pub seperators: Vec<usize>,
}

/// Represents a modifier for a noun, containing position, compound elements, siblings, and associated linguistic elements.
#[derive(Default, Clone, Ord, PartialOrd, PartialEq, Eq)]
pub struct NounModifier {
    pub position: usize,
    pub compound_elements: Vec<usize>,
    pub siblings: Vec<NounSibling>,
    pub prepositions: Vec<usize>,
    pub determiners: Vec<usize>,
    pub adjectives: Vec<Adjective>,
}

/// Represents a sibling noun, with position, exclusion status, determiners, adjectives, and separators.
#[derive(Default, Clone, Ord, PartialOrd, PartialEq, Eq)]
pub struct NounSibling {
    pub position: usize,
    pub is_excluded: bool,
    pub determiners: Vec<usize>,
    pub adjectives: Vec<Adjective>,
    pub seperators: Vec<usize>,
}

/// Represents the owner of a noun, indicating none, speaker, listener, or a third-party individual or group.
#[derive(Clone, PartialEq, Default)]
pub enum NounOwner {
    #[default]
    none,
    speaker,
    listener,
    third_individual(String),
    third_group(String),
}

/// Represents a verb with a head token, objects, modifiers, siblings, and associated linguistic elements.
#[derive(Default, Clone)]
pub struct Verb {
    pub head: usize,
    pub objects: Vec<usize>,
    pub modifiers: Vec<VerbModifier>,
    pub siblings: Vec<VerbSibling>,
    pub auxillary_verbs: Vec<usize>,
    pub prepositions: Vec<usize>,
    pub determiners: Vec<usize>,
    pub adverbs: Vec<Adverb>,
    pub seperators: Vec<usize>,
    pub linkers: Vec<usize>,
}

/// Represents a modifier for a verb, containing position, siblings, objects, and associated linguistic elements.
#[derive(Default, Clone, Ord, PartialOrd, PartialEq, Eq)]
pub struct VerbModifier {
    pub position: usize,
    pub siblings: Vec<VerbSibling>,
    pub objects: Vec<usize>,
    pub auxillary_verbs: Vec<usize>,
    pub prepositions: Vec<usize>,
    pub adverbs: Vec<Adverb>,
}

/// Represents a sibling verb, with position, objects, and separators.
#[derive(Default, Clone, Ord, PartialOrd, PartialEq, Eq)]
pub struct VerbSibling {
    pub position: usize,
    pub objects: Vec<usize>,
    pub seperators: Vec<usize>,
}

/// Represents an adjective with position, categories, associated adverbs, and predicative verbs.
#[derive(Default, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Adjective {
    pub position: usize,
    pub categories: Vec<String>,
    pub adverbs: Vec<Adverb>,
    pub predicative_verbs: Vec<usize>,
}

/// Represents an adverb with position and associated categories.
#[derive(Default, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Adverb {
    pub position: usize,
    pub categories: Vec<String>,
}

/// Represents the tense of a phrase, which can be undetermined, past, present, or future.
#[derive(Clone, PartialEq, Default)]
pub enum PhraseTense {
    #[default]
    undetermined,
    past,
    present,
    future,
}

/// Represents the person of a phrase, which can be undetermined, first, second, or third.
#[derive(Clone, PartialEq, Default)]
pub enum PhrasePerson {
    #[default]
    undetermined,
    first,
    second,
    third,
}

/// Represents the classification of a phrase, which can be undetermined, imperative, interrogative, declarative, conversational, or exclamatory.
#[derive(Clone, PartialEq, Default)]
pub enum PhraseClassification {
    #[default]
    undetermined,
    imperative,
    interrogative,
    declarative,
    conversational,
    exclamatory,
}

impl Phrase {
    /// Creates a new Phrase instance with a specified start position and optional split token.
    pub fn new(start: &usize, split_token: Option<usize>) -> Self {
        Self {
            range: *start..*start,
            split_token,
            ..Default::default()
        }
    }

    /// Converts the phrase to a string representation using the provided tokens, including split token if present.
    pub fn to_string(&self, tokens: &Vec<Token>) -> String {
        let split_word = match &self.split_token {
            Some(r) => format!(" [{}]", tokens[*r].word),
            None => String::new(),
        };

        let words = tokens[self.range.clone()]
            .iter()
            .map(|token| {
                if let Some(antecedent) = &token.antecedent {
                    format!("{} [{}]", token.word, antecedent)
                } else {
                    token.word.to_string()
                }
            })
            .collect::<Vec<String>>();

        format!("{}{}", words.join(" "), split_word)
    }

    /// Converts the phrase to a detailed debug string, including nouns and verbs, using the provided tokens.
    pub fn to_debug_string(&self, tokens: &Vec<Token>) -> String {
        let mut lines = vec![self.to_string(tokens), String::new()];

        for noun in self.nouns.iter() {
            let noun_str = format!("    noun: {}", noun.to_string(tokens));
            lines.push(noun_str);
        }

        for verb in self.verbs.iter() {
            let verb_str = format!("    verb: {}", verb.to_string(tokens));
            lines.push(verb_str);
        }
        lines.join("\n")
    }
}

impl Noun {
    /// Adds a compound noun by updating the head or appending to the last modifier's compound elements.
    pub fn add_compound(&mut self, noun: &Noun) {
        if let Some(modifier) = self.modifiers.last_mut() {
            modifier.compound_elements.push(modifier.position);
            modifier.position = noun.head;
        } else {
            self.compound_elements.push(self.head);
            self.head = noun.head;
        }
    }

    /// Adds a sibling noun to either the noun's siblings or the last modifier's siblings.
    pub fn add_sibling(&mut self, noun: &Noun) {
        if let Some(modifier) = self.modifiers.last_mut() {
            modifier.siblings.push(noun.to_sibling());
        } else {
            self.siblings.push(noun.to_sibling());
        }
    }

    /// Converts the noun to a NounModifier, preserving relevant fields.
    pub fn to_modifier(&self) -> NounModifier {
        NounModifier {
            position: self.head,
            compound_elements: Vec::new(),
            siblings: Vec::new(),
            prepositions: self.prepositions.clone(),
            determiners: self.determiners.clone(),
            adjectives: self.adjectives.clone(),
        }
    }

    /// Converts the noun to a NounSibling, preserving relevant fields.
    pub fn to_sibling(&self) -> NounSibling {
        NounSibling {
            position: self.head,
            is_excluded: false,
            determiners: self.determiners.clone(),
            adjectives: self.adjectives.clone(),
            seperators: self.seperators.clone(),
        }
    }

    /// Retrieves the first modifier with prepositions, if any.
    pub fn get_preposition_modifier(&self) -> Option<NounModifier> {
        for modifier in self.modifiers.iter() {
            if !modifier.prepositions.is_empty() {
                return Some(modifier.clone());
            }
        }
        None
    }

    /// Returns the leftmost position among the noun's head, modifiers, determiners, prepositions, and adjectives.
    pub fn get_left_most_position(&self) -> usize {
        let mut pos = self.head;

        if let Some(new_pos) = self.modifiers.iter().filter(|m| m.position < pos).min() {
            pos = new_pos.position;
        }

        if let Some(new_pos) = self.determiners.iter().filter(|m| **m < pos).min() {
            pos = *new_pos;
        }

        if let Some(new_pos) = self.prepositions.iter().filter(|m| **m < pos).min() {
            pos = *new_pos;
        }

        if let Some(new_pos) = self.adjectives.iter().filter(|adv| adv.position < pos).min() {
            pos = new_pos.position;
        }

        pos
    }

    /// Returns the rightmost position among the noun's head, modifiers, siblings, and adjectives.
    pub fn get_right_most_position(&self) -> usize {
        let mut pos = self.head;

        if let Some(new_pos) = self.modifiers.iter().filter(|m| m.position > pos).max() {
            pos = new_pos.position;
        }

        if let Some(new_pos) = self.siblings.iter().filter(|m| m.position > pos).max() {
            pos = new_pos.position;
        }

        if let Some(new_pos) = self.adjectives.iter().filter(|adv| adv.position > pos).max() {
            pos = new_pos.position;
        }
        pos
    }

    /// Converts the noun to a string representation, including compounds, prepositions, determiners, adjectives, siblings, and modifiers.
    pub fn to_string(&self, tokens: &Vec<Token>) -> String {
        let mut elem = vec![tokens[self.head].word.to_string()];

        // compounds
        if !self.compound_elements.is_empty() {
            let line = format!(
                "[comp: {}]",
                self.compound_elements
                    .iter()
                    .map(|x| tokens[*x].word.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            );
            elem.push(line);
        }

        // Prepositions
        if !self.prepositions.is_empty() {
            let line = format!(
                "[pp: {}]",
                self.prepositions
                    .iter()
                    .map(|x| tokens[*x].word.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            );
            elem.push(line);
        }

        // Determiners
        if !self.determiners.is_empty() {
            let line = format!(
                "[dt: {}]",
                self.determiners
                    .iter()
                    .map(|x| tokens[*x].word.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            );
            elem.push(line);
        }

        // Adjectives
        for adj in self.adjectives.iter() {
            let adj_str = format!("[adj: {}]", adj.to_string(tokens));
            elem.push(adj_str);
        }
        let mut res = vec![elem.join(" ")];

        // Siblings
        for sib in self.siblings.iter() {
            let mod_str = format!("        sibling: {}", sib.to_string(tokens));
            res.push(mod_str);
        }

        // Modifiers
        for modifier in self.modifiers.iter() {
            let mod_str = format!("        modifier: {}", modifier.to_string(tokens));
            res.push(mod_str);
        }

        res.join("\n")
    }
}

impl NounSibling {
    /// Converts the noun sibling to a string representation, including exclusion status, determiners, adjectives, and separators.
    pub fn to_string(&self, tokens: &Vec<Token>) -> String {
        // Start elements
        let mut elem = if self.is_excluded {
            vec![format!(
                "{} (excl.)",
                tokens[self.position].word.to_string()
            )]
        } else {
            vec![format!(
                "{} (incl.)",
                tokens[self.position].word.to_string()
            )]
        };

        // Determiners
        if !self.determiners.is_empty() {
            let line = format!(
                "[dt: {}]",
                self.determiners
                    .iter()
                    .map(|x| tokens[*x].word.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            );
            elem.push(line);
        }

        // Adjectives
        for adj in self.adjectives.iter() {
            let adj_str = format!("[adj: {}]", adj.to_string(tokens));
            elem.push(adj_str);
        }

        // seperators
        if !self.seperators.is_empty() {
            let line = format!(
                "[sep: {}]",
                self.seperators
                    .iter()
                    .map(|x| tokens[*x].word.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            );
            elem.push(line);
        }

        elem.join(" ")
    }
}

impl NounModifier {
    /// Converts the noun modifier to a noun, optionally including a modifier.
    pub fn to_noun(&self, modifier: Option<Noun>) -> Noun {
        Noun {
            head: self.position,
            modifiers: if modifier.is_none() {
                vec![]
            } else {
                vec![modifier.unwrap().to_modifier()]
            },
            prepositions: self.prepositions.clone(),
            determiners: self.determiners.clone(),
            adjectives: self.adjectives.clone(),
            ..Default::default()
        }
    }

    /// Converts the noun modifier to a string representation, including compounds, prepositions, determiners, adjectives, and siblings.
    pub fn to_string(&self, tokens: &Vec<Token>) -> String {
        // Start elements
        let mut elem = vec![tokens[self.position].word.to_string()];

        // compounds
        if !self.compound_elements.is_empty() {
            let line = format!(
                "[comp: {}]",
                self.compound_elements
                    .iter()
                    .map(|x| tokens[*x].word.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            );
            elem.push(line);
        }

        // prepositions
        if !self.prepositions.is_empty() {
            let line = format!(
                "[pp: {}]",
                self.prepositions
                    .iter()
                    .map(|x| tokens[*x].word.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            );
            elem.push(line);
        }

        // Determiners
        if !self.determiners.is_empty() {
            let line = format!(
                "[dt: {}]",
                self.determiners
                    .iter()
                    .map(|x| tokens[*x].word.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            );
            elem.push(line);
        }

        // Adjectives
        for adj in self.adjectives.iter() {
            let adj_str = format!("[adj: {}]", adj.to_string(tokens));
            elem.push(adj_str);
        }
        let mut res = vec![elem.join(" ")];

        // Siblings
        for sib in self.siblings.iter() {
            let mod_str = format!("            mod sib: {}", sib.to_string(tokens));
            res.push(mod_str);
        }

        res.join("\n")
    }
}

impl Adverb {
    /// Creates a new Adverb instance with position and categories derived from the token and vocabulary database.
    pub fn new(x: usize, token: &Token, vocab: &VocabDatabase) -> Self {
        let categories = token
            .categories
            .iter()
            .map(|cat_id| match vocab.categories.nodes.get(cat_id) {
                Some(cat) => cat.name.to_string(),
                None => String::new(),
            })
            .collect::<Vec<String>>();

        Self {
            position: x,
            categories,
        }
    }
}

impl Verb {
    /// Converts the verb to a VerbModifier, preserving relevant fields.
    pub fn to_modifier(&self) -> VerbModifier {
        VerbModifier {
            position: self.head,
            siblings: Vec::new(),
            objects: self.objects.clone(),
            auxillary_verbs: self.auxillary_verbs.clone(),
            prepositions: self.prepositions.clone(),
            adverbs: self.adverbs.clone(),
        }
    }

    /// Converts the verb to a VerbSibling, preserving relevant fields.
    pub fn to_sibling(&self) -> VerbSibling {
        VerbSibling {
            position: self.head,
            objects: self.objects.clone(),
            seperators: self.seperators.clone(),
        }
    }

    /// Adds a sibling verb to either the verb's siblings or the last modifier's siblings.
    pub fn add_sibling(&mut self, verb: &Verb) {
        if let Some(modifier) = self.modifiers.last_mut() {
            modifier.siblings.push(verb.to_sibling());
        } else {
            self.siblings.push(verb.to_sibling());
        }
    }

    /// Returns the leftmost position among the verb's head, modifiers, auxiliary verbs, prepositions, and adverbs.
    pub fn get_left_most_position(&self) -> usize {
        let mut pos = self.head;
        if let Some(new_pos) = self.modifiers.iter().filter(|m| m.position < pos).min() {
            pos = new_pos.position;
        }

        if let Some(new_pos) = self.auxillary_verbs.iter().filter(|m| **m < pos).min() {
            pos = *new_pos;
        }

        if let Some(new_pos) = self.prepositions.iter().filter(|m| **m < pos).min() {
            pos = *new_pos;
        }

        if let Some(new_pos) = self.adverbs.iter().filter(|adv| adv.position < pos).min() {
            pos = new_pos.position;
        }

        pos
    }

    /// Returns the rightmost position among the verb's head and adverbs.
    pub fn get_right_most_position(&self) -> usize {
        let mut pos = self.head;

        if let Some(new_pos) = self.adverbs.iter().filter(|adv| adv.position > pos).max() {
            pos = new_pos.position;
        }

        pos
    }

    /// Converts the verb to a string representation, including auxiliary verbs, prepositions, adverbs, objects, siblings, and modifiers.
    pub fn to_string(&self, tokens: &Vec<Token>) -> String {
        // Start
        let mut elem = vec![tokens[self.head].word.to_string()];

        // auxillary verbs
        if !self.auxillary_verbs.is_empty() {
            let line = format!(
                "[aux: {}]",
                self.auxillary_verbs
                    .iter()
                    .map(|x| tokens[*x].word.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            );
            elem.push(line);
        }

        // prepositions
        if !self.prepositions.is_empty() {
            let line = format!(
                "[pp: {}]",
                self.prepositions
                    .iter()
                    .map(|x| tokens[*x].word.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            );
            elem.push(line);
        }

        // Adverbs
        if !self.adverbs.is_empty() {
            let line = format!(
                "[adv: {}]",
                self.adverbs
                    .iter()
                    .map(|x| tokens[x.position].word.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            );
            elem.push(line);
        }

        // objects
        if !self.objects.is_empty() {
            let line = format!(
                "[obj: {}]",
                self.objects
                    .iter()
                    .map(|x| tokens[*x].word.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            );
            elem.push(line);
        }
        let mut res = vec![elem.join(" ")];

        // Siblings
        for sibling in self.siblings.iter() {
            let line = format!("        sibling: {}", sibling.to_string(tokens));
            res.push(line);
        }

        // Modifiers
        for modifier in self.modifiers.iter() {
            let line = format!("        modifier: {}", modifier.to_string(tokens));
            res.push(line);
        }

        res.join("\n")
    }
}

impl VerbModifier {
    /// Converts the verb modifier to a string representation, including auxiliary verbs, prepositions, adverbs, objects, and siblings.
    pub fn to_string(&self, tokens: &Vec<Token>) -> String {
        // Start
        let mut elem = vec![tokens[self.position].word.to_string()];

        // auxillary verbs
        if !self.auxillary_verbs.is_empty() {
            let line = format!(
                "[aux: {}]",
                self.auxillary_verbs
                    .iter()
                    .map(|x| tokens[*x].word.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            );
            elem.push(line);
        }

        // prepositions
        if !self.prepositions.is_empty() {
            let line = format!(
                "[pp: {}]",
                self.prepositions
                    .iter()
                    .map(|x| tokens[*x].word.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            );
            elem.push(line);
        }

        // Adverbs
        if !self.adverbs.is_empty() {
            let line = format!(
                "[adv: {}]",
                self.adverbs
                    .iter()
                    .map(|x| tokens[x.position].word.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            );
            elem.push(line);
        }

        // objects
        if !self.objects.is_empty() {
            let line = format!(
                "[obj: {}]",
                self.objects
                    .iter()
                    .map(|x| tokens[*x].word.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            );
            elem.push(line);
        }
        let mut res = vec![elem.join(" ")];

        // Siblings
        for sib in self.siblings.iter() {
            let mod_str = format!("            mod sib: {}", sib.to_string(tokens));
            res.push(mod_str);
        }

        res.join("\n")
    }
}

impl VerbSibling {
    /// Converts the verb sibling to a string representation, including objects and separators.
    pub fn to_string(&self, tokens: &Vec<Token>) -> String {
        // Start
        let mut elem = vec![tokens[self.position].word.to_string()];

        // objects
        if !self.objects.is_empty() {
            let line = format!(
                "[obj: {}]",
                self.objects
                    .iter()
                    .map(|x| tokens[*x].word.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            );
            elem.push(line);
        }

        // seperators
        if !self.seperators.is_empty() {
            let line = format!(
                "[sep: {}]",
                self.seperators
                    .iter()
                    .map(|x| tokens[*x].word.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            );
            elem.push(line);
        }

        elem.join(" ")
    }
}

impl Adjective {
    /// Creates a new Adjective instance with position and categories derived from the token and vocabulary database.
    pub fn new(x: usize, token: &Token, vocab: &VocabDatabase) -> Self {
        let categories = token
            .categories
            .iter()
            .map(|cat_id| match vocab.categories.nodes.get(cat_id) {
                Some(cat) => cat.name.to_string(),
                None => String::new(),
            })
            .collect::<Vec<String>>();

        Self {
            position: x,
            categories,
            ..Default::default()
        }
    }

    /// Converts the adjective to a string representation, including predicative verbs and adverbs.
    pub fn to_string(&self, tokens: &Vec<Token>) -> String {
        let mut elem = vec![tokens[self.position].word.to_string()];

        // predicative_verbs
        if !self.predicative_verbs.is_empty() {
            let line = format!(
                "[pred: {}]",
                self.predicative_verbs
                    .iter()
                    .map(|x| tokens[*x].word.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            );
            elem.push(line);
        }

        // adverbs
        if !self.adverbs.is_empty() {
            let line = format!(
                "[adv: {}]",
                self.adverbs
                    .iter()
                    .map(|x| tokens[x.position].word.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            );
            elem.push(line);
        }

        elem.join(" ")
    }
}

impl fmt::Display for NounOwner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res = match self {
            NounOwner::none => "none".to_string(),
            NounOwner::speaker => "speaker".to_string(),
            NounOwner::listener => "listener".to_string(),
            NounOwner::third_individual(name) => format!("Person: {}", name),
            NounOwner::third_group(name) => format!("Group: {}", name),
        };
        write!(f, "{}", res)
    }
}

impl fmt::Display for PhraseTense {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res = match self {
            PhraseTense::past => "past",
            PhraseTense::present => "present",
            PhraseTense::future => "future",
            _ => "undetermined",
        };
        write!(f, "{}", res)
    }
}

impl fmt::Display for PhrasePerson {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res = match self {
            PhrasePerson::first => "first",
            PhrasePerson::second => "second",
            PhrasePerson::third => "third",
            _ => "undetermined",
        };
        write!(f, "{}", res)
    }
}

impl fmt::Display for PhraseClassification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res = match self {
            PhraseClassification::declarative => "declarative",
            PhraseClassification::imperative => "imperative",
            PhraseClassification::interrogative => "interrogative",
            PhraseClassification::exclamatory => "exclamatory",
            PhraseClassification::conversational => "conversational",
            _ => "undetermined",
        };
        write!(f, "{}", res)
    }
}
