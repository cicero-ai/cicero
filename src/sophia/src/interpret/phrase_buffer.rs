// Copyright 2025 Aquila Labs of Alberta, Canada <matt@cicero.sh>
// Licensed under the PolyForm Noncommercial License 1.0.0
// Commercial use requires a separate license: https://cicero.sh/sophia/
// License text: https://polyformproject.org/licenses/noncommercial/1.0.0/
// Distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND.

use super::{AntecedentBuffer, CoreferenceCategories};
use crate::interpret::phrase::{
    Adjective, Adverb, Noun, Phrase, PhraseClassification, PhrasePerson, PhraseTense, Verb,
};
use crate::pos_tagger::POSTag;
use crate::tokenizer::Token;
use crate::vocab::{PhraseIntent, PronounCategory, PronounPerson, VocabDatabase};
use std::collections::HashMap;
use std::ops::Range;

#[derive(Default)]
pub struct PhraseBuffer {
    pub antecedents: AntecedentBuffer,
    pub state_verb_category: Range<i16>,
    pub degree_adverb_category: Range<i16>,
    pub place_adverb_category: Range<i16>,
    pub phrases: Vec<Phrase>,
    pub tokens: Vec<Token>,
    pub phrase: Phrase,
    pub checkpoint: usize,
    pub prev_checkpoint: usize,
    pub last_pos: POSTag,
    pub last_push: (char, usize),
    pub determiners: Vec<usize>,
    pub prepositions: Vec<usize>,
    pub noun_seperators: Vec<usize>,
    pub auxillary_verbs: Vec<usize>,
    pub predicative_verbs: Vec<usize>,
    pub subjects: Vec<usize>,
    pub adjectives: Vec<Adjective>,
    pub adverbs: Vec<Adverb>,
    pub splitters: Vec<usize>,
    pub linkers: Vec<usize>,
    pub noise: Vec<usize>,
    pub current_nouns: Vec<Noun>,
    pub current_verbs: Vec<Verb>,
    pub intents: Vec<IntentMarker>,
}

pub struct IntentMarker {
    pub intent: PhraseIntent,
    pub length: usize,
}

impl PhraseBuffer {
    /// Creates a new PhraseBuffer instance from the provided coreference categories and vocabulary database.
    pub fn new(coref: &CoreferenceCategories, vocab: &VocabDatabase) -> Self {
        Self {
            antecedents: AntecedentBuffer::new(coref),
            state_verb_category: vocab.categories.verbs.path2range("state").unwrap_or(0..0),
            degree_adverb_category: vocab.categories.adverbs.path2range("degree").unwrap_or(0..0),
            place_adverb_category: vocab.categories.adverbs.path2range("place").unwrap_or(0..0),
            phrase: Phrase::new(&0, None),
            ..Default::default()
        }
    }

    /// Creates a new Verb instance with current buffer state and clears relevant buffer fields.
    pub fn create_verb(&mut self, x: usize) -> Verb {
        let verb = Verb {
            head: x,
            prepositions: self.prepositions.clone(),
            determiners: self.determiners.clone(),
            auxillary_verbs: self.auxillary_verbs.clone(),
            adverbs: self.adverbs.clone(),
            seperators: self.noun_seperators.clone(),
            ..Default::default()
        };

        self.prepositions.clear();
        self.determiners.clear();
        self.auxillary_verbs.clear();
        self.adverbs.clear();
        self.noun_seperators.clear();
        verb
    }

    /// Creates a new Noun instance with current buffer state and clears relevant buffer fields.
    pub fn create_noun(&mut self, x: usize) -> Noun {
        let noun = Noun {
            head: x,
            prepositions: self.prepositions.clone(),
            determiners: self.determiners.clone(),
            adjectives: self.adjectives.clone(),
            seperators: self.noun_seperators.clone(),
            ..Default::default()
        };

        self.determiners.clear();
        self.prepositions.clear();
        self.adjectives.clear();
        self.noun_seperators.clear();
        noun
    }

    /// Adds a noun to the buffer, pre-processing if needed, creating a noun, post-processing, and updating antecedents.
    pub fn add_noun(&mut self, x: usize) {
        // Pre-process buffer, if needed
        if self.current_nouns.is_empty() {
            self.pre_process(x);
        } else {
            self.pre_process_left_noun();
        }

        // Add noun
        let noun = self.create_noun(x);
        self.post_process(x);
        self.current_nouns.push(noun);

        // Add to antecedent buffer
        self.antecedents.add_noun(&self.tokens[x]);
    }

    /// Adds a verb to the buffer, pre-processing if needed, creating a verb, and post-processing.
    pub fn add_verb(&mut self, x: usize) {
        // Pre-process buffer, if needed
        if self.current_verbs.is_empty() {
            self.pre_process(x);
        } else {
            //self.pre_process_left_verb(x);
        }

        // Add verb
        let verb = self.create_verb(x);
        self.post_process(x);
        self.current_verbs.push(verb);
    }

    /// Adds a pronoun to the buffer, resolving it against antecedents and updating phrase person if applicable.
    pub fn add_pronoun(&mut self, x: usize) {
        // Check pronoun category
        let cat = self.tokens[x].pronoun.clone().unwrap().category;
        if ![
            PronounCategory::personal,
            PronounCategory::possessive,
            PronounCategory::reflexive,
        ]
        .contains(&cat)
        {
            self.noise.push(x);
            return;
        }

        // Resolve
        self.antecedents.resolve_pronoun(&mut self.tokens[x]);
        let pronoun = self.tokens[x].pronoun.clone().unwrap();

        // Set phrase person
        if pronoun.person == PronounPerson::first {
            self.phrase.person = PhrasePerson::first;
        } else if self.tokens[x].word.as_str() == "you" {
            self.phrase.person = PhrasePerson::second;
        } else if self.phrase.person == PhrasePerson::undetermined
            && pronoun.person == PronounPerson::third
        {
            self.phrase.person = PhrasePerson::third;
        }
    }

    /// Adds a phrase intent to the buffer
    pub fn add_intent(&mut self, intent: PhraseIntent, length: usize) {
        self.intents.push(IntentMarker { intent, length });
    }

    /// Pre-processes the buffer based on the current token, handling verbs or nouns and processing prior phrases if needed.
    pub fn pre_process(&mut self, x: usize) {
        // Left word is a verb
        if self.tokens[self.checkpoint].is_verb() && !self.current_verbs.is_empty() {
            // Check for adjective
            if !self.phrase.verbs.is_empty() {
                self.pre_process_left_verb(x);
            }

            // Process previous verb phrase
            self.process_verb_phrase();
        }

        // Left word is a noun
        if self.tokens[self.checkpoint].is_noun() && !self.current_nouns.is_empty() {
            // Adjective proceeding  noun
            if !self.adjectives.is_empty() {
                self.pre_process_left_noun();
            }

            // Process previous noun phrase
            self.process_noun_phrase();
        }
    }

    /// Pre-processes the left-side verb, assigning linkers and adverbs based on context and token type.
    pub fn pre_process_left_verb(&mut self, x: usize) {
        // Get last verb
        let verb = match self.phrase.verbs.last_mut() {
            Some(r) => r,
            None => return,
        };

        verb.linkers.extend(&self.linkers.clone());
        self.linkers.clear();

        // No adjectives, and right word is noun
        if self.adjectives.is_empty() && self.tokens[x].is_noun() {
            verb.adverbs.extend(self.adverbs.clone());
            self.adverbs.clear();
            return;
        }

        // Get necessary positions
        let adj_pos = self.adjectives.first().unwrap_or(&Adjective::default()).position;
        let pp_pos = self.prepositions.first().unwrap_or(&0);

        // Go through adverbs
        for adv in self.adverbs.iter() {
            // Right word is verb, and before any preposition
            if self.tokens[x].is_verb() && (*pp_pos == 0 || *pp_pos > adv.position) {
                verb.adverbs.push(adv.clone());

            // Before adjective, and not a manner adverb
            } else if adj_pos > 0
                && adv.position < adj_pos
                && !adv.categories.contains(&"manner".to_string())
            {
                verb.adverbs.push(adv.clone());
            }
        }
        self.adverbs.retain(|a| !verb.adverbs.iter().any(|ac| ac.position == a.position));
    }

    /// Pre-processes the left-side noun, assigning adjectives and related adverbs/predicative verbs.
    pub fn pre_process_left_noun(&mut self) {
        // Get last noun
        let noun = match self.current_nouns.last_mut() {
            Some(r) => r,
            None => return,
        };

        // Go through adjectives
        for adj in self.adjectives.iter_mut() {
            // Check if stopper exists
            if self.splitters.iter().any(|p| *p < adj.position) {
                break;
            }

            // Ensure predicative verb preceeds adjective
            if !self.predicative_verbs.iter().any(|p| *p < adj.position) {
                break;
            }
            adj.predicative_verbs.push(self.predicative_verbs.remove(0));

            // Check for adverb
            let adv = self.adverbs.first().unwrap_or(&Adverb::default()).clone();
            if adv.position > 0 && adv.position < adj.position {
                adj.adverbs.push(self.adverbs.remove(0));
            }

            // Add adjective to noun
            noun.adjectives.push(adj.clone());
        }
        self.adjectives
            .retain(|adj| !noun.adjectives.iter().any(|adjc| adj.position == adjc.position));
    }

    /// Processes a group of nouns, handling compounds, siblings, and modifiers, and pushing the final noun to the phrase.
    fn process_noun_phrase(&mut self) {
        // Initialize
        let mut head = self.current_nouns.remove(0);
        let has_head = true;

        // Check if we have a conjunction
        let has_conjunction = self
            .current_nouns
            .iter()
            .any(|noun| noun.seperators.iter().any(|sep| self.tokens[*sep].pos == POSTag::CC));

        // Go through nouns
        while !self.current_nouns.is_empty() {
            let noun = self.current_nouns.remove(0);

            // Compound noun
            if noun.prepositions.is_empty()
                && noun.determiners.is_empty()
                && noun.seperators.is_empty()
            {
                head.add_compound(&noun);
            } else if has_conjunction && !noun.seperators.is_empty() {
                head.add_sibling(&noun);
                //head.siblings.push(noun.to_sibling());
            } else if head.prepositions.is_empty() && !noun.prepositions.is_empty() {
                head.modifiers.push(noun.to_modifier());
            //} else if (!noun.prepositions.is_empty()) && head.modifiers.iter().any(|m| !m.prepositions.is_empty()) {
            //let modifier = head.get_preposition_modifier().unwrap();
            //self.push_noun(modifier.to_noun(Some(noun.clone())));
            //head = noun;
            } else {
                self.push_noun(head);
                head = noun;
            }
        }

        if has_head {
            self.push_noun(head.clone());
        }
        self.current_nouns.clear();
    }

    /// Processes a verb phrase, handling linked verbs, modifiers, and siblings, and pushing the final verb to the phrase.
    fn process_verb_phrase(&mut self) {
        // Filter separate verbs away
        let mut linked_verbs = Vec::new();
        let mut head = self.current_verbs.remove(0);

        // Check if we have a conjunction
        let has_conjunction = self
            .current_verbs
            .iter()
            .any(|verb| verb.seperators.iter().any(|sep| self.tokens[*sep].pos == POSTag::CC));

        // Go through verbs
        while !self.current_verbs.is_empty() {
            let verb = self.current_verbs.remove(0);

            if !verb.prepositions.is_empty() {
                head.modifiers.push(verb.to_modifier());
            } else if verb.prepositions.is_empty() && has_conjunction && !verb.seperators.is_empty()
            {
                head.add_sibling(&verb);
            } else if !verb.determiners.is_empty() {
                self.push_verb(head);
                head = verb;
            } else {
                linked_verbs.push(verb);
            }
        }

        // Get linked verbs
        while let Some(verb) = linked_verbs.pop() {
            head.modifiers.push(verb.to_modifier());
        }
        self.push_verb(head);
        self.current_verbs.clear();
    }

    /// Post-processes the buffer by updating checkpoints and clearing temporary fields.
    pub fn post_process(&mut self, x: usize) {
        self.prev_checkpoint = self.checkpoint;
        self.checkpoint = x;

        self.determiners.clear();
        self.prepositions.clear();
        self.noun_seperators.clear();
        self.auxillary_verbs.clear();
        self.predicative_verbs.clear();
        self.adjectives.clear();
        self.adverbs.clear();
        self.splitters.clear();
        self.linkers.clear();
        self.noise.clear();
    }

    /// Finalizes the buffer by pre-processing, processing remaining nouns/verbs, and either appending to the last phrase or splitting.
    pub fn hard_split(&mut self, x: usize) {
        // Preprocess
        self.pre_process(x);

        if !self.current_nouns.is_empty() {
            self.process_noun_phrase();
        }

        if !self.current_verbs.is_empty() {
            self.pre_process_left_verb(x);
        }

        // Clear buffer
        self.post_process(x);

        // Append to previous phrase, or split
        if (self.phrase.verbs.is_empty()
            || (self.phrase.nouns.is_empty()
                && !self.phrase.verbs.iter().any(|v| !v.objects.is_empty())))
            && !self.phrases.is_empty()
        {
            let last_phrase = self.phrases.last_mut().unwrap();
            last_phrase.split_token = None;
            last_phrase.range.end = x + 1;
            last_phrase.nouns.extend(std::mem::take(&mut self.phrase.nouns));
            last_phrase.verbs.extend(std::mem::take(&mut self.phrase.verbs));
        } else {
            self.do_split(x);
        }
    }

    /// Checks if a phrase split is needed based on verbs, nouns, and token types, triggering a split if conditions are met.
    fn check_split(&mut self, end: usize, current_pos: usize) {
        // Check verb and nouns
        if self.phrase.verbs.is_empty() || self.phrase.nouns.is_empty() {
            return;
        }

        // Get start position
        let start = if self.last_push.0 == 'N' {
            self.phrase.nouns[self.phrase.nouns.len() - 1].get_right_most_position()
        } else {
            self.phrase.verbs[self.phrase.verbs.len() - 1].get_right_most_position()
        };

        // Check for single past verb
        if self.phrase.verbs.len() == 1
            && self.tokens[self.phrase.verbs[0].head].is_past_verb()
            && self.tokens[current_pos].is_noun()
        {
            return;
        }
        if start >= end {
            return;
        }

        // preposition
        if let Some(pos) =
            self.tokens[start..end + 1].iter().position(|token| token.pos == POSTag::IN)
            && self.tokens[pos].word.as_str() != "of"
        {
            self.do_split(pos + start);
            return;
        }

        // Look for common or semi-colon
        if let Some(pos) = self.tokens[start..end + 1]
            .iter()
            .position(|token| [",", ";"].contains(&token.word.as_str()))
        {
            self.do_split(pos + start);
            return;
        }

        // Adverb
        if let Some(pos) = self.tokens[start..end + 1]
            .iter()
            .position(|token| token.pos.to_str().starts_with("RB"))
        {
            self.do_split(pos + start);
            return;
        }

        // determiner
        if let Some(pos) =
            self.tokens[start..end + 1].iter().position(|token| token.pos == POSTag::DT)
        {
            self.do_split(pos + start);
            return;
        }

        self.do_split(start);
    }

    /// Performs a phrase split, classifying the current phrase and creating a new one with updated properties.
    fn do_split(&mut self, pos: usize) {
        // Check for end of tokens
        if self.phrase.range.start >= pos {
            return;
        }

        // Classify phrase
        self.classify_phrase();

        // Define new phrase
        let mut new_phrase = Phrase::new(&(pos + 1), None);
        if self.tokens[pos].word.as_str() != "|nl|" {
            new_phrase.person = self.phrase.person.clone();
            new_phrase.tense = PhraseTense::undetermined;
            new_phrase.classification = self.phrase.classification.clone();
        }

        // Get intent
        self.phrase.intent = self.score_intent(&self.intents, self.tokens.len());
        self.intents.clear();

        // Split phrase
        self.phrase.range.end = pos;
        self.phrase.split_token = Some(pos);
        self.phrases.push(std::mem::replace(&mut self.phrase, new_phrase));
    }

    /// Score vector of intents
    pub fn score_intent(
        &self,
        markers: &[IntentMarker],
        total_tokens: usize,
    ) -> (PhraseIntent, f32) {
        let mut counts: HashMap<PhraseIntent, usize> = HashMap::new();
        for marker in markers.iter() {
            *counts.entry(marker.intent).or_insert(0) += marker.length;
        }

        let (mut max_intent, mut max_score) = (PhraseIntent::neutral, 0.0);
        for (intent, count) in counts.iter() {
            let score: f32 = *count as f32 / total_tokens as f32;
            if score > max_score {
                max_score = score;
                max_intent = *intent;
            }
        }

        (max_intent, max_score)
    }

    /// Pushes a noun to the phrase, pre-processing verbs if needed and checking for splits.
    fn push_noun(&mut self, noun: Noun) {
        // Process left side of verb
        if self.last_push.0 == 'V' {
            self.pre_process_left_verb(noun.head);
        }
        self.check_split(noun.get_left_most_position(), noun.head);
        self.last_push = ('N', noun.head);
        self.phrase.nouns.push(noun);
    }

    /// Pushes a verb to the phrase, pre-processing nouns if needed, classifying tense, and checking for splits.
    fn push_verb(&mut self, verb: Verb) {
        // Process left side of noun
        if self.last_push.0 == 'N' {
            self.pre_process_left_noun();
        }

        // Split if necessary
        self.check_split(verb.get_left_most_position(), verb.head);
        self.last_push = ('V', verb.head);

        // lassify tense
        if self.phrase.tense == PhraseTense::undetermined {
            if self.tokens[verb.head].is_past_verb() {
                self.phrase.tense = PhraseTense::past;
            } else if self.tokens[verb.head].is_future_verb() {
                self.phrase.tense = PhraseTense::future;
            } else {
                self.phrase.tense = PhraseTense::present;
            }
        }

        // Push verb
        self.phrase.verbs.push(verb);
    }

    /// Classifies the current phrase based on person and tense, setting conversational, declarative, or resolved classification.
    pub fn classify_phrase(&mut self) {
        // Imply second person, if no pronouns found
        if self.phrase.person == PhrasePerson::undetermined {
            self.phrase.person = PhrasePerson::second;
        }

        // Set classification
        if self.phrase.person == PhrasePerson::first && self.phrase.tense == PhraseTense::past {
            self.phrase.classification = PhraseClassification::conversational;
        } else if self.phrase.person == PhrasePerson::third
            && self.phrase.tense == PhraseTense::past
        {
            self.phrase.classification = PhraseClassification::conversational;
        } else if self.phrase.person == PhrasePerson::third {
            self.phrase.classification = PhraseClassification::declarative;
        } else {
            self.phrase.classification = self.resolve_classification();
        }
    }

    /// Resolves phrase classification based on token properties, such as question marks, verbs, or adverbs.
    fn resolve_classification(&mut self) -> PhraseClassification {
        // Check for empty token
        if self.tokens.is_empty() {
            return PhraseClassification::undetermined;
        }

        // Check for question mark
        if self.tokens.last().unwrap().word.as_str() == "?"
            || ["who", "what", "where", "why", "why"].contains(&self.tokens[0].word.as_str())
        {
            return PhraseClassification::interrogative;
        }

        // Declarative
        if self
            .tokens
            .iter()
            .any(|token| token.is_verb() && token.has_category(&self.state_verb_category))
        {
            return PhraseClassification::declarative;
        } else if self
            .tokens
            .iter()
            .any(|token| token.is_adverb() && token.has_category(&self.degree_adverb_category))
        {
            return PhraseClassification::declarative;
        } else if self
            .tokens
            .iter()
            .any(|token| token.is_adverb() && token.has_category(&self.place_adverb_category))
        {
            return PhraseClassification::conversational;
        }

        PhraseClassification::imperative
    }
}
