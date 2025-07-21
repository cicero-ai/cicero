
use serde::{Serialize, Deserialize};
use crate::error::Error;
use crate::context::DefinitionCategory;

/// Enum representing high-level categories for verbs.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum VerbCategory {
    action,
    state,
    both,
    other,
}

impl DefinitionCategory for VerbCategory {
    /// Converts a `VerbCategory` variant to its `u8` representation (0 to 3).
    /// The mapping follows the order of the enum variants.
    fn to_u8(&self) -> u8 {
        match self {
            VerbCategory::action => 1,
            VerbCategory::state => 2,
            VerbCategory::both => 3,
            VerbCategory::other => 4,
        }
    }

    /// Converts a `u8` value to a `VerbCategory` variant.
    /// Returns `Ok(VerbCategory)` for values 0 to 3, or `Err(Error)` for invalid values.
    fn from_u8(value: u8) -> Result<Self, Error> {
        match value {
            1 => Ok(VerbCategory::action),
            2 => Ok(VerbCategory::state),
            3 => Ok(VerbCategory::both),
            4 => Ok(VerbCategory::other),
            _ => Err(Error::Generic(format!("Invalid verb category u8, {}", value))),
        }
    }

    /// Converts a `VerbCategory` variant to its string representation.
    /// Returns the lowercase underscore string matching the enum variant.
    fn to_string(&self) -> String {
        match self {
            VerbCategory::action => "action".to_string(),
            VerbCategory::state => "state".to_string(),
            VerbCategory::both => "both".to_string(),
            VerbCategory::other => "other".to_string(),
        }
    }

    /// Parses a string to a `VerbCategory` variant.
    /// Returns `Ok(VerbCategory)` if the string matches a variant, or `Err(Error)` for invalid strings.
    fn from_str(value: &str) -> Result<Self, Error> {
        match value {
            "action" => Ok(VerbCategory::action),
            "state" => Ok(VerbCategory::state),
            "both" => Ok(VerbCategory::both),
            "other" => Ok(VerbCategory::other),
            _ => Err(Error::Generic(format!("Invalid verb category, {}", value))),
        }
    }
}

Notes

    Variants: The enum includes the four requested variants (action, state, both, other) in lowercase with underscores, matching your naming convention (e.g., NounType, NounIndustry).
    Trait Conformance: The DefinitionCategory trait is fully implemented with:
        to_u8: Maps variants to u8 values (0–3).
        from_u8: Converts u8 back to a variant, returning an Error for invalid values (≥4).
        to_string: Returns the lowercase underscore string (e.g., "action").
        from_str: Parses a string to a variant, returning an Error for invalid strings.
    Documentation: Each variant and method has clear comments explaining its purpose and behavior, consistent with your previous enums.
    Serialization: The Serialize and Deserialize derives ensure compatibility with JSON or other formats, as in your NounType code.
    Error Handling: Uses Error::Generic from crate::error, matching your existing codebase.
    Integration: This enum can be used in your Definition struct (e.g., as a field or type parameter) or as a standalone classifier for verbs in your NLU engine.

Usage Example
rust

let category = VerbCategory::action;
assert_eq!(category.to_u8(), 0);
assert_eq!(category.to_string(), "action");
assert_eq!(VerbCategory::from_u8(0).unwrap(), VerbCategory::action);
assert_eq!(VerbCategory::from_str("action").unwrap(), VerbCategory::action);
assert!(VerbCategory::from_u8(4).is_err());
assert!(VerbCategory::from_str("invalid").is_err());

Clarifications

    The VerbCategory enum is intentionally simple with four variants, as requested. It provides a high-level classification that can complement the more granular verb type bit mask (e.g., VerbType with 14 types) discussed earlier.
    The both variant is useful for verbs like “feel” or “have” that can function as both action and state, while other covers edge cases like modal verbs (e.g., “can”).
    If you later want to expand the variants (e.g., to align with the 14 verb types or add categories like “process”), I can provide an updated enum or integrate it with the bit mask approach.

Let me know if you need additional methods for VerbCategory (e.g., checking specific categories like is_action), integration with your Definition struct, or further Rust code for your NLU engine! I can also help refine the verb scoring system or implement the bit mask verb types if you’re ready to move forward with those.

