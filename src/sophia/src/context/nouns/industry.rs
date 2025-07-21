
use serde::{Serialize, Deserialize};
use crate::error::Error;

/// Enum representing different industries associated with nouns.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum NounIndustry {
    astronomy,
    biology,
    chemistry,
    physics,
    geology,
    medicine,
    culinary,
    agriculture,
    technology,
    engineering,
    construction,
    transportation,
    military,
    arts,
    entertainment,
    music,
    sports,
    fashion,
    finance,
    education,
    religion,
    politics,
    environment,
    communication,
    manufacturing,
    energy,
    law,
    hospitality,
    retail,
    aerospace,
    maritime,
    research,
}

impl NounIndustry {
    /// Converts a `NounIndustry` variant to its `u8` representation (0 to 31).
    /// The mapping follows the order of the enum variants, starting at 0.
    pub fn to_u8(&self) -> u8 {
        match self {
            NounIndustry::astronomy => 0,
            NounIndustry::biology => 1,
            NounIndustry::chemistry => 2,
            NounIndustry::physics => 3,
            NounIndustry::geology => 4,
            NounIndustry::medicine => 5,
            NounIndustry::culinary => 6,
            NounIndustry::agriculture => 7,
            NounIndustry::technology => 8,
            NounIndustry::engineering => 9,
            NounIndustry::construction => 10,
            NounIndustry::transportation => 11,
            NounIndustry::military => 12,
            NounIndustry::arts => 13,
            NounIndustry::entertainment => 14,
            NounIndustry::music => 15,
            NounIndustry::sports => 16,
            NounIndustry::fashion => 17,
            NounIndustry::finance => 18,
            NounIndustry::education => 19,
            NounIndustry::religion => 20,
            NounIndustry::politics => 21,
            NounIndustry::environment => 22,
            NounIndustry::communication => 23,
            NounIndustry::manufacturing => 24,
            NounIndustry::energy => 25,
            NounIndustry::law => 26,
            NounIndustry::hospitality => 27,
            NounIndustry::retail => 28,
            NounIndustry::aerospace => 29,
            NounIndustry::maritime => 30,
            NounIndustry::research => 31,
        }
    }

    /// Converts a `u8` value to a `NounIndustry` variant.
    /// Returns `Ok(NounIndustry)` for values 0 to 31, or `Err(Error)` for invalid values.
    pub fn from_u8(value: u8) -> Result<Self, Error> {
        match value {
            0 => Ok(NounIndustry::astronomy),
            1 => Ok(NounIndustry::biology),
            2 => Ok(NounIndustry::chemistry),
            3 => Ok(NounIndustry::physics),
            4 => Ok(NounIndustry::geology),
            5 => Ok(NounIndustry::medicine),
            6 => Ok(NounIndustry::culinary),
            7 => Ok(NounIndustry::agriculture),
            8 => Ok(NounIndustry::technology),
            9 => Ok(NounIndustry::engineering),
            10 => Ok(NounIndustry::construction),
            11 => Ok(NounIndustry::transportation),
            12 => Ok(NounIndustry::military),
            13 => Ok(NounIndustry::arts),
            14 => Ok(NounIndustry::entertainment),
            15 => Ok(NounIndustry::music),
            16 => Ok(NounIndustry::sports),
            17 => Ok(NounIndustry::fashion),
            18 => Ok(NounIndustry::finance),
            19 => Ok(NounIndustry::education),
            20 => Ok(NounIndustry::religion),
            21 => Ok(NounIndustry::politics),
            22 => Ok(NounIndustry::environment),
            23 => Ok(NounIndustry::communication),
            24 => Ok(NounIndustry::manufacturing),
            25 => Ok(NounIndustry::energy),
            26 => Ok(NounIndustry::law),
            27 => Ok(NounIndustry::hospitality),
            28 => Ok(NounIndustry::retail),
            29 => Ok(NounIndustry::aerospace),
            30 => Ok(NounIndustry::maritime),
            31 => Ok(NounIndustry::research),
            _ => Err(Error::Generic(format!("Invalid industry u8, {}", value))),
        }
    }
}


