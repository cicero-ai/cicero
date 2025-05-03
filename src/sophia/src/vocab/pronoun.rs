use serde::{Deserialize, Serialize};

/// Represents a pronoun with its linguistic properties, including category, gender, person, and number.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Pronoun {
    pub category: PronounCategory,
    pub sub_category: PronounCategory,
    pub gender: PronounGender,
    pub person: PronounPerson,
    pub number: PronounNumber,
}

/// Defines the category of a pronoun, such as personal, possessive, or indefinite.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum PronounCategory {
    none,
    personal,
    possessive,
    indefinite,
    reflexive,
    demonstrative,
    interrogative,
    relative,
}

/// Defines the gender of a pronoun, which can be neutral, male, or female.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum PronounGender {
    neutral,
    male,
    female,
}

/// Defines the person of a pronoun, which can be neutral, first, second, or third.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum PronounPerson {
    neutral,
    first,
    second,
    third,
}

/// Defines the number of a pronoun, which can be neutral, singular, or plural.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum PronounNumber {
    neutral,
    singular,
    plural,
}

impl Pronoun {
    /// Checks if the pronoun requires anaphora resolution, based on its category and person.
    pub fn is_anaphora(&self) -> bool {
        if ![
            PronounCategory::personal,
            PronounCategory::possessive,
            PronounCategory::reflexive,
        ]
        .contains(&self.category)
        {
            return false;
        }

        if self.person == PronounPerson::first {
            return false;
        }

        true
    }
}
