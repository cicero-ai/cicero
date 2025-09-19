// Copyright 2025 Aquila Labs of Alberta, Canada <matt@cicero.sh>
// Licensed under the PolyForm Noncommercial License 1.0.0
// Commercial use requires a separate license: https://cicero.sh/sophia/
// License text: https://polyformproject.org/licenses/noncommercial/1.0.0/
// Distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND.

use super::CoreferenceCategories;
use crate::pos_tagger::POSTag;
use crate::tokenizer::Token;
use crate::vocab::{Pronoun, PronounCategory, PronounGender, PronounNumber, PronounPerson};
use std::collections::HashSet;

/// Manages antecedents for coreference resolution, tracking primary and secondary antecedents, person counts, and plural references.
#[derive(Default)]
pub struct AntecedentBuffer {
    coref: CoreferenceCategories,
    count: usize,
    primary: Antecedent,
    secondary: Vec<Antecedent>,
    last_person: String,
    plural_person: HashSet<String>,
    primary_object: String,
}

/// Represents an antecedent with a name, part-of-speech tag, type (person, entity, object), and gender.
#[derive(Debug, Clone)]
struct Antecedent {
    name: String,
    pos: String,
    antecedent_type: AntecedentType,
    gender: PronounGender,
}

/// Defines the type of an antecedent, which can be none, person, entity, or object.
#[derive(Debug, Clone, PartialEq)]
enum AntecedentType {
    none,
    person,
    entity,
    object,
}

impl AntecedentBuffer {
    /// Creates a new AntecedentBuffer with the provided coreference categories.
    pub fn new(coref: &CoreferenceCategories) -> Self {
        Self {
            coref: coref.clone(),
            ..Default::default()
        }
    }

    /// Adds a noun token to the antecedent buffer, classifying it as person, entity, or object based on coreference rules.
    pub fn add_noun(&mut self, token: &Token) {
        // Check for person / object
        if self.coref.is_person(token) {
            self.add(&token.word, &token.pos, AntecedentType::person);
            self.last_person = token.word.to_string();
            self.plural_person.insert(token.word.to_string());
        } else if self.coref.is_entity(token) {
            self.add(&token.word, &token.pos, AntecedentType::entity);
        } else if token.is_noun() {
            self.add(&token.word, &token.pos, AntecedentType::object);
        }
    }

    /// Adds an antecedent to the buffer, updating primary or secondary lists and setting primary object if applicable.
    fn add(&mut self, name: &str, pos: &POSTag, antecedent_type: AntecedentType) {
        let ant = Antecedent {
            name: name.to_string(),
            pos: pos.to_str(),
            antecedent_type,
            gender: PronounGender::neutral,
        };

        if ant.antecedent_type == AntecedentType::object && self.primary_object.is_empty() {
            self.primary_object = name.to_string();
        }

        if ant.antecedent_type == AntecedentType::person
            && self.primary.antecedent_type == AntecedentType::none
        {
            self.primary = ant;
        } else {
            self.secondary.push(ant);
        }
    }

    /// Adds a non-noun token to the buffer, resetting plural person tracking for verbs/prepositions or clearing the buffer if needed.
    pub fn add_non_noun(&mut self, token: &Token) {
        if token.is_verb() || token.is_preposition() {
            self.plural_person = HashSet::new();
        }

        // Clear, if needed
        if self.count >= 30 || token.word.as_str() == "|nl|" {
            self.clear();
        } else {
            self.count += 1;
        }
    }

    /// Resolves a pronoun in the token by assigning an antecedent based on gender, number, and person, if applicable.
    pub fn resolve_pronoun(&mut self, token: &mut Token) {
        // Get pronoun
        let pronoun = match &token.pronoun {
            Some(r) => r,
            None => return,
        };
        if !pronoun.is_anaphora() {
            return;
        }
        self.count = 0;

        // Ensure third person
        if pronoun.person == PronounPerson::first || pronoun.person == PronounPerson::second {
            return;
        }

        // Get antecedent
        if pronoun.gender != PronounGender::neutral && pronoun.number == PronounNumber::singular {
            token.antecedent = self.get_singular_person(pronoun);
        } else if pronoun.gender == PronounGender::neutral
            && pronoun.number == PronounNumber::singular
        {
            token.antecedent = if self.primary_object.is_empty() {
                None
            } else {
                Some(self.primary_object.to_string())
            };
        } else if pronoun.number == PronounNumber::plural {
            token.antecedent = self.get_plural(pronoun);
        }
    }

    /// Resolves a singular person pronoun, matching gender and updating the primary or secondary antecedent.
    fn get_singular_person(&mut self, pronoun: &Pronoun) -> Option<String> {
        // First person, or no primary
        if self.primary.antecedent_type == AntecedentType::none {
            return None;
        }

        // Possessive
        if (pronoun.category == PronounCategory::possessive
            || self.last_person != self.primary.name)
            && (self.primary.gender == pronoun.gender
                || self.primary.gender == PronounGender::neutral)
        {
            if self.primary.gender == PronounGender::neutral {
                self.primary.gender = pronoun.gender.clone();
            }
            //self.last_person = self.primary.name.to_string();
            return Some(self.primary.name.to_string());
        }

        // Go through names, identify correct gender
        let mut name = String::new();
        for elem in self.secondary.iter_mut().rev() {
            if elem.antecedent_type != AntecedentType::person {
                continue;
            } else if elem.gender == pronoun.gender {
                name = elem.name.to_string();
                break;
            } else if elem.gender == PronounGender::neutral {
                elem.gender = pronoun.gender.clone();
                name = elem.name.to_string();
                break;
            }
        }

        // No name found
        if name.is_empty() {
            return None;
        }

        self.last_person = name.to_string();
        Some(name)
    }

    /// Resolves a third-person plural pronoun, prioritizing plural persons or falling back to entities/objects.
    fn get_plural(&mut self, _pronoun: &Pronoun) -> Option<String> {
        // Try for person
        if let Some(name) = self.get_plural_person() {
            return Some(name);
        }

        // Look for entity or object
        let mut res: Option<String> = None;
        for elem in self.secondary.iter().rev() {
            if elem.antecedent_type == AntecedentType::person {
                continue;
            }
            if elem.antecedent_type == AntecedentType::entity
                || (elem.antecedent_type == AntecedentType::object && elem.pos.as_str() == "NP")
            {
                res = Some(elem.name.to_string());
                break;
            }
        }

        res
    }

    /// Retrieves a third-person plural person antecedent by combining primary and secondary persons, if available.
    fn get_plural_person(&mut self) -> Option<String> {
        if self.plural_person.len() >= 2 && self.plural_person.contains(&self.primary.name) {
            return Some(self.plural_person.iter().cloned().collect::<Vec<_>>().join("|"));
        } else if self.primary.name.is_empty() {
            return None;
        }

        // Look for person in buffer
        let mut people = vec![self.primary.name.to_string()];
        for elem in self.secondary.iter().rev() {
            if elem.antecedent_type != AntecedentType::person {
                continue;
            }
            if elem.name != self.primary.name {
                people.push(elem.name.to_string());
                break;
            }
        }

        // Return, if we have two
        if people.len() > 1 {
            return Some(people.join("|"));
        }

        None
    }

    /// Clears the antecedent buffer, resetting all fields to their default state.
    fn clear(&mut self) {
        self.count = 0;
        self.primary = Antecedent::default();
        self.secondary = Vec::new();
        self.last_person = String::new();
        self.plural_person = HashSet::new();
        self.primary_object = String::new();
    }
}

impl Default for Antecedent {
    fn default() -> Antecedent {
        Antecedent {
            name: String::new(),
            pos: String::new(),
            antecedent_type: AntecedentType::none,
            gender: PronounGender::neutral,
        }
    }
}
