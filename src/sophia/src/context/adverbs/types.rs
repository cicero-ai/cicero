
use serde::{Serialize, Deserialize};
use crate::error::Error;

/// Enum representing categories for adverbs based on their semantic or syntactic role.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum AdverbCategory {
    /// Adverbs indicating contrast or unexpected outcomes (e.g., nevertheless, however).
    concession,
    /// Adverbs specifying conditions or hypothetical scenarios (e.g., possibly, potentially).
    condition,
    /// Adverbs indicating extent or intensity (e.g., very, extremely).
    degree,
    /// Adverbs indicating how often something occurs (e.g., always, often).
    frequency,
    /// Adverbs describing how an action is performed (e.g., quickly, carefully).
    manner,
    /// Adverbs indicating location or spatial context (e.g., here, everywhere).
    place,
    /// Adverbs indicating intention or goal (e.g., intentionally, purposely).
    purpose,
    /// Adverbs explaining why something occurs (e.g., therefore, consequently).
    reason,
    /// Adverbs indicating when or over what period something occurs (e.g., now, yesterday).
    time,
    /// Adverbs that do not fit other categories (e.g., just, only).
    uncategorized,
}

impl DefinitionCategory for AdverbCategory {
    /// Converts an `AdverbCategory` variant to its `u8` representation (0 to 9).
    /// The mapping follows the order of the enum variants.
    fn to_u8(&self) -> u8 {
        match self {
            AdverbCategory::concession => 0,
            AdverbCategory::condition => 1,
            AdverbCategory::degree => 2,
            AdverbCategory::frequency => 3,
            AdverbCategory::manner => 4,
            AdverbCategory::place => 5,
            AdverbCategory::purpose => 6,
            AdverbCategory::reason => 7,
            AdverbCategory::time => 8,
            AdverbCategory::uncategorized => 9,
        }
    }

    /// Converts a `u8` value to an `AdverbCategory` variant.
    /// Returns `Ok(AdverbCategory)` for values 0 to 9, or `Err(Error)` for invalid values.
    fn from_u8(value: u8) -> Result<Self, Error> {
        match value {
            0 => Ok(AdverbCategory::concession),
            1 => Ok(AdverbCategory::condition),
            2 => Ok(AdverbCategory::degree),
            3 => Ok(AdverbCategory::frequency),
            4 => Ok(AdverbCategory::manner),
            5 => Ok(AdverbCategory::place),
            6 => Ok(AdverbCategory::purpose),
            7 => Ok(AdverbCategory::reason),
            8 => Ok(AdverbCategory::time),
            9 => Ok(AdverbCategory::uncategorized),
            _ => Err(Error::Generic(format!("Invalid adverb category u8, {}", value))),
        }
    }

    /// Converts an `AdverbCategory` variant to its string representation.
    /// Returns the lowercase underscore string matching the enum variant.
    fn to_string(&self) -> String {
        match self {
            AdverbCategory::concession => "concession".to_string(),
            AdverbCategory::condition => "condition".to_string(),
            AdverbCategory::degree => "degree".to_string(),
            AdverbCategory::frequency => "frequency".to_string(),
            AdverbCategory::manner => "manner".to_string(),
            AdverbCategory::place => "place".to_string(),
            AdverbCategory::purpose => "purpose".to_string(),
            AdverbCategory::reason => "reason".to_string(),
            AdverbCategory::time => "time".to_string(),
            AdverbCategory::uncategorized => "uncategorized".to_string(),
        }
    }

    /// Parses a string to an `AdverbCategory` variant.
    /// Returns `Ok(AdverbCategory)` if the string matches a variant, or `Err(Error)` for invalid strings.
    fn from_str(value: &str) -> Result<Self, Error> {
        match value {
            "concession" => Ok(AdverbCategory::concession),
            "condition" => Ok(AdverbCategory::condition),
            "degree" => Ok(AdverbCategory::degree),
            "frequency" => Ok(AdverbCategory::frequency),
            "manner" => Ok(AdverbCategory::manner),
            "place" => Ok(AdverbCategory::place),
            "purpose" => Ok(AdverbCategory::purpose),
            "reason" => Ok(AdverbCategory::reason),
            "time" => Ok(AdverbCategory::time),
            "uncategorized" => Ok(AdverbCategory::uncategorized),
            _ => Err(Error::Generic(format!("Invalid adverb category, {}", value))),
        }
    }
}

