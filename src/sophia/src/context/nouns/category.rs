
use serde::{Serialize, Deserialize};
use crate::context::DefinitionCategory;
use crate::error::Error;

/// Enum representing different types of nouns
#[derive(Default, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum NounCategory {
    living_creature,
    plant_based_organism,
    micro_organism,
    human_body,
    natural_material,
    manufactured_object,
    vehicle,
    building,
    weapon,
    chemical_element,
    fluid_gas,
    composite_material,
    geographic_feature,
    geographic_location,
    relative_location,
    food,
    celestial_body,
    defined_space,
    philosophical_concept,
    scientific_principle,
    mathematical_concept,
    theory_idea,
    natural_system,
    human_made_system,
    technical_system,
    methodology,
    natural_event,
    social_event,
    historical_event,
    phenomenon,
    knowledge_unit,
    document_media,
    communication,
    occupation,
    institution_organization,
    cultural_belief_system,
    time,
    #[default]
    unknown,
}


impl DefinitionCategory for NounCategory {
    /// Converts a `NounType` variant to its string representation.
    /// Returns the lowercase underscore string matching the enum variant.
    fn to_string(&self) -> String {
        match self {
            Self::living_creature => "living_creature".to_string(),
            Self::plant_based_organism => "plant_based_organism".to_string(),
            Self::micro_organism => "micro_organism".to_string(),
            Self::human_body => "human_body".to_string(),
            Self::natural_material => "natural_material".to_string(),
            Self::manufactured_object => "manufactured_object".to_string(),
            Self::vehicle => "vehicle".to_string(),
            Self::building => "building".to_string(),
            Self::weapon => "weapon".to_string(),
            Self::chemical_element => "chemical_element".to_string(),
            Self::fluid_gas => "fluid_gas".to_string(),
            Self::composite_material => "composite_material".to_string(),
            Self::geographic_feature => "geographic_feature".to_string(),
            Self::geographic_location => "geographic_location".to_string(),
            Self::relative_location => "relative_location".to_string(),
            Self::food => "food".to_string(),
            Self::celestial_body => "celestial_body".to_string(),
            Self::defined_space => "defined_space".to_string(),
            Self::philosophical_concept => "philosophical_concept".to_string(),
            Self::scientific_principle => "scientific_principle".to_string(),
            Self::mathematical_concept => "mathematical_concept".to_string(),
            Self::theory_idea => "theory_idea".to_string(),
            Self::natural_system => "natural_system".to_string(),
            Self::human_made_system => "human_made_system".to_string(),
            Self::technical_system => "technical_system".to_string(),
            Self::methodology => "methodology".to_string(),
            Self::natural_event => "natural_event".to_string(),
            Self::social_event => "social_event".to_string(),
            Self::historical_event => "historical_event".to_string(),
            Self::phenomenon => "phenomenon".to_string(),
            Self::knowledge_unit => "knowledge_unit".to_string(),
            Self::document_media => "document_media".to_string(),
            Self::communication => "communication".to_string(),
            Self::occupation => "occupation".to_string(),
            Self::institution_organization => "institution_organization".to_string(),
            Self::cultural_belief_system => "cultural_belief_system".to_string(),
            Self::time => "time".to_string(),
            Self::unknown => "unknown".to_string(),
        }
    }

    /// Parses a string to a `NounType` variant.
    /// Returns `Ok(NounType)` if the string matches a variant, or `Err(Error)` for invalid strings.
    fn from_str(value: &str) -> Result<Self, Error> {
        match value {
            "living_creature" => Ok(Self::living_creature),
            "plant_based_organism" => Ok(Self::plant_based_organism),
            "micro_organism" => Ok(Self::micro_organism),
            "human_body" => Ok(Self::human_body),
            "natural_material" => Ok(Self::natural_material),
            "manufactured_object" => Ok(Self::manufactured_object),
            "vehicle" => Ok(Self::vehicle),
            "building" => Ok(Self::building),
            "weapon" => Ok(Self::weapon),
            "chemical_element" => Ok(Self::chemical_element),
            "fluid_gas" => Ok(Self::fluid_gas),
            "composite_material" => Ok(Self::composite_material),
            "geographic_feature" => Ok(Self::geographic_feature),
            "geographic_location" => Ok(Self::geographic_location),
            "relative_location" => Ok(Self::relative_location),
            "food" => Ok(Self::food),
            "celestial_body" => Ok(Self::celestial_body),
            "defined_space" => Ok(Self::defined_space),
            "philosophical_concept" => Ok(Self::philosophical_concept),
            "scientific_principle" => Ok(Self::scientific_principle),
            "mathematical_concept" => Ok(Self::mathematical_concept),
            "theory_idea" => Ok(Self::theory_idea),
            "natural_system" => Ok(Self::natural_system),
            "human_made_system" => Ok(Self::human_made_system),
            "technical_system" => Ok(Self::technical_system),
            "methodology" => Ok(Self::methodology),
            "natural_event" => Ok(Self::natural_event),
            "social_event" => Ok(Self::social_event),
            "historical_event" => Ok(Self::historical_event),
            "phenomenon" => Ok(Self::phenomenon),
            "knowledge_unit" => Ok(Self::knowledge_unit),
            "document_media" => Ok(Self::document_media),
            "communication" => Ok(Self::communication),
            "occupation" => Ok(Self::occupation),
            "institution_organization" => Ok(Self::institution_organization),
            "cultural_belief_system" => Ok(Self::cultural_belief_system),
            "time" => Ok(Self::time),
            "unknown" => Ok(Self::unknown),
            _ => Err(Error::Generic(format!("Invalid noun type, {}", value))),
        }
    }

    /// Converts a `NounType` variant to a `u8` value (0 to 36) based on its order.
    /// The mapping follows the order of the enum variants.
    fn to_u8(&self) -> u8 {
        match self {
            Self::living_creature => 1,
            Self::plant_based_organism => 2,
            Self::micro_organism => 3,
            Self::human_body => 4,
            Self::natural_material => 5,
            Self::manufactured_object => 6,
            Self::vehicle => 7,
            Self::building => 8,
            Self::weapon => 9,
            Self::chemical_element => 10,
            Self::fluid_gas => 11,
            Self::composite_material => 12,
            Self::geographic_feature => 13,
            Self::geographic_location => 14,
            Self::relative_location => 15,
            Self::food => 16,
            Self::celestial_body => 17,
            Self::defined_space => 18,
            Self::philosophical_concept => 19,
            Self::scientific_principle => 20,
            Self::mathematical_concept => 21,
            Self::theory_idea => 22,
            Self::natural_system => 23,
            Self::human_made_system => 24,
            Self::technical_system => 25,
            Self::methodology => 26,
            Self::natural_event => 27,
            Self::social_event => 28,
            Self::historical_event => 29,
            Self::phenomenon => 30,
            Self::knowledge_unit => 31,
            Self::document_media => 32,
            Self::communication => 33,
            Self::occupation => 34,
            Self::institution_organization => 35,
            Self::cultural_belief_system => 36,
            Self::time => 37,
            Self::unknown => 38,
        }
    }

    /// Converts a `u8` value to a `NounType` variant.
    /// Returns `Ok(NounType)` for values 1 to 37, or `Err(Error)` for invalid values.
    fn from_u8(value: u8) -> Result<Self, Error> {
        match value {
            1 => Ok(Self::living_creature),
            2 => Ok(Self::plant_based_organism),
            3 => Ok(Self::micro_organism),
            4 => Ok(Self::human_body),
            5 => Ok(Self::natural_material),
            6 => Ok(Self::manufactured_object),
            7 => Ok(Self::vehicle),
            8 => Ok(Self::building),
            9 => Ok(Self::weapon),
            10 => Ok(Self::chemical_element),
            11 => Ok(Self::fluid_gas),
            12 => Ok(Self::composite_material),
            13 => Ok(Self::geographic_feature),
            14 => Ok(Self::geographic_location),
            15 => Ok(Self::relative_location),
            16 => Ok(Self::food),
            17 => Ok(Self::celestial_body),
            18 => Ok(Self::defined_space),
            19 => Ok(Self::philosophical_concept),
            20 => Ok(Self::scientific_principle),
            21 => Ok(Self::mathematical_concept),
            22 => Ok(Self::theory_idea),
            23 => Ok(Self::natural_system),
            24 => Ok(Self::human_made_system),
            25 => Ok(Self::technical_system),
            26 => Ok(Self::methodology),
            27 => Ok(Self::natural_event),
            28 => Ok(Self::social_event),
            29 => Ok(Self::historical_event),
            30 => Ok(Self::phenomenon),
            31 => Ok(Self::knowledge_unit),
            32 => Ok(Self::document_media),
            33 => Ok(Self::communication),
            34 => Ok(Self::occupation),
            35 => Ok(Self::institution_organization),
            36 => Ok(Self::cultural_belief_system),
            37 => Ok(Self::time),
            38 => Ok(Self::unknown),
            _ => Err(Error::Generic(format!("Invalid noun type u8, {}", value))),
        }
    }

    /// Get schema, that defines how scores are stored based on noun category
    ///   I = integer, F = float, B = boolean, M = bit mask, Z = null
    fn get_schema(&self) -> String {
        "IIFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFMMMMMM".to_string()
    }

}


