
use serde::{Serialize, Deserialize};
use crate::error::Error;

/// Enum representing categories for adjectives based on their semantic role in describing nouns.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum AdjectiveCategory {
    /// Adjectives describing temporal age (e.g., young, old).
    age,
    /// Adjectives describing beauty or style (e.g., beautiful, elegant).
    aesthetics,
    /// Adjectives describing hue or shade (e.g., red, bright).
    color,
    /// Adjectives describing physical or functional state (e.g., broken, pristine).
    condition,
    /// Adjectives describing feelings or mood (e.g., happy, sad).
    emotion,
    /// Adjectives describing utility or purpose (e.g., useful, decorative).
    function,
    /// Adjectives describing spatial position (e.g., nearby, distant).
    location,
    /// Adjectives describing composition (e.g., wooden, metallic).
    material,
    /// Adjectives describing ethical quality (e.g., good, evil).
    morality,
    /// Adjectives describing amount or extent (e.g., many, few).
    quantity,
    /// Adjectives describing source or nationality (e.g., French, ancient).
    origin,
    /// Adjectives describing abstract qualities (e.g., difficult, unique).
    quality,
    /// Adjectives describing form or structure (e.g., round, square).
    shape,
    /// Adjectives describing visibility or appearance (e.g., clear, faint).
    visual_clarity,
    /// Adjectives describing dimensions or magnitude (e.g., big, small).
    size,
    /// Adjectives describing odor (e.g., fragrant, foul).
    smell,
    /// Adjectives describing auditory properties (e.g., loud, quiet).
    sound,
    /// Adjectives describing flavor (e.g., sweet, bitter).
    taste,
    /// Adjectives describing heat or cold (e.g., hot, cold).
    temperature,
    /// Adjectives describing surface feel (e.g., smooth, rough).
    texture,
}

impl DefinitionCategory for AdjectiveCategory {
    /// Converts an `AdjectiveCategory` variant to its `u8` representation (0 to 19).
    /// The mapping follows the order of the enum variants.
    fn to_u8(&self) -> u8 {
        match self {
            AdjectiveCategory::age => 0,
            AdjectiveCategory::aesthetics => 1,
            AdjectiveCategory::color => 2,
            AdjectiveCategory::condition => 3,
            AdjectiveCategory::emotion => 4,
            AdjectiveCategory::function => 5,
            AdjectiveCategory::location => 6,
            AdjectiveCategory::material => 7,
            AdjectiveCategory::morality => 8,
            AdjectiveCategory::quantity => 9,
            AdjectiveCategory::origin => 10,
            AdjectiveCategory::quality => 11,
            AdjectiveCategory::shape => 12,
            AdjectiveCategory::visual_clarity => 13,
            AdjectiveCategory::size => 14,
            AdjectiveCategory::smell => 15,
            AdjectiveCategory::sound => 16,
            AdjectiveCategory::taste => 17,
            AdjectiveCategory::temperature => 18,
            AdjectiveCategory::texture => 19,
        }
    }

    /// Converts a `u8` value to an `AdjectiveCategory` variant.
    /// Returns `Ok(AdjectiveCategory)` for values 0 to 19, or `Err(Error)` for invalid values.
    fn from_u8(value: u8) -> Result<Self, Error> {
        match value {
            0 => Ok(AdjectiveCategory::age),
            1 => Ok(AdjectiveCategory::aesthetics),
            2 => Ok(AdjectiveCategory::color),
            3 => Ok(AdjectiveCategory::condition),
            4 => Ok(AdjectiveCategory::emotion),
            5 => Ok(AdjectiveCategory::function),
            6 => Ok(AdjectiveCategory::location),
            7 => Ok(AdjectiveCategory::material),
            8 => Ok(AdjectiveCategory::morality),
            9 => Ok(AdjectiveCategory::quantity),
            10 => Ok(AdjectiveCategory::origin),
            11 => Ok(AdjectiveCategory::quality),
            12 => Ok(AdjectiveCategory::shape),
            13 => Ok(AdjectiveCategory::visual_clarity),
            14 => Ok(AdjectiveCategory::size),
            15 => Ok(AdjectiveCategory::smell),
            16 => Ok(AdjectiveCategory::sound),
            17 => Ok(AdjectiveCategory::taste),
            18 => Ok(AdjectiveCategory::temperature),
            19 => Ok(AdjectiveCategory::texture),
            _ => Err(Error::Generic(format!("Invalid adjective category u8, {}", value))),
        }
    }

    /// Converts an `AdjectiveCategory` variant to its string representation.
    /// Returns the lowercase underscore string matching the enum variant.
    fn to_string(&self) -> String {
        match self {
            AdjectiveCategory::age => "age".to_string(),
            AdjectiveCategory::aesthetics => "aesthetics".to_string(),
            AdjectiveCategory::color => "color".to_string(),
            AdjectiveCategory::condition => "condition".to_string(),
            AdjectiveCategory::emotion => "emotion".to_string(),
            AdjectiveCategory::function => "function".to_string(),
            AdjectiveCategory::location => "location".to_string(),
            AdjectiveCategory::material => "material".to_string(),
            AdjectiveCategory::morality => "morality".to_string(),
            AdjectiveCategory::quantity => "quantity".to_string(),
            AdjectiveCategory::origin => "origin".to_string(),
            AdjectiveCategory::quality => "quality".to_string(),
            AdjectiveCategory::shape => "shape".to_string(),
            AdjectiveCategory::visual_clarity => "visual_clarity".to_string(),
            AdjectiveCategory::size => "size".to_string(),
            AdjectiveCategory::smell => "smell".to_string(),
            AdjectiveCategory::sound => "sound".to_string(),
            AdjectiveCategory::taste => "taste".to_string(),
            AdjectiveCategory::temperature => "temperature".to_string(),
            AdjectiveCategory::texture => "texture".to_string(),
        }
    }

    /// Parses a string to an `AdjectiveCategory` variant.
    /// Returns `Ok(AdjectiveCategory)` if the string matches a variant, or `Err(Error)` for invalid strings.
    fn from_str(value: &str) -> Result<Self, Error> {
        match value {
            "age" => Ok(AdjectiveCategory::age),
            "aesthetics" => Ok(AdjectiveCategory::aesthetics),
            "color" => Ok(AdjectiveCategory::color),
            "condition" => Ok(AdjectiveCategory::condition),
            "emotion" => Ok(AdjectiveCategory::emotion),
            "function" => Ok(AdjectiveCategory::function),
            "location" => Ok(AdjectiveCategory::location),
            "material" => Ok(AdjectiveCategory::material),
            "morality" => Ok(AdjectiveCategory::morality),
            "quantity" => Ok(AdjectiveCategory::quantity),
            "origin" => Ok(AdjectiveCategory::origin),
            "quality" => Ok(AdjectiveCategory::quality),
            "shape" => Ok(AdjectiveCategory::shape),
            "visual_clarity" => Ok(AdjectiveCategory::visual_clarity),
            "size" => Ok(AdjectiveCategory::size),
            "smell" => Ok(AdjectiveCategory::smell),
            "sound" => Ok(AdjectiveCategory::sound),
            "taste" => Ok(AdjectiveCategory::taste),
            "temperature" => Ok(AdjectiveCategory::temperature),
            "texture" => Ok(AdjectiveCategory::texture),
            _ => Err(Error::Generic(format!("Invalid adjective category, {}", value))),
        }
    }
}

