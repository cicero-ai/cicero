
use serde::{Serialize, Deserialize};
use crate::error::Error;

/// Enum representing possible states of matter combinations for nouns.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum NounState {
    none,
    plasma_only,
    gas_only,
    gas_and_plasma,
    liquid_only,
    liquid_and_plasma,
    liquid_and_gas,
    liquid_gas_and_plasma,
    solid_only,
    solid_and_plasma,
    solid_and_gas,
    solid_gas_and_plasma,
    solid_and_liquid,
    solid_liquid_and_plasma,
    solid_liquid_and_gas,
    solid_liquid_gas_and_plasma,
}

/// Individual state of matter for use in vector representation.
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum MatterState {
    Solid,
    Liquid,
    Gas,
    Plasma,
}

impl NounState {
    /// Converts a `NounState` variant to its `u8` representation (0 to 15).
    /// The mapping follows the order of the enum variants.
    pub fn to_u8(&self) -> u8 {
        match self {
            NounState::none => 0,
            NounState::plasma_only => 1,
            NounState::gas_only => 2,
            NounState::gas_and_plasma => 3,
            NounState::liquid_only => 4,
            NounState::liquid_and_plasma => 5,
            NounState::liquid_and_gas => 6,
            NounState::liquid_gas_and_plasma => 7,
            NounState::solid_only => 8,
            NounState::solid_and_plasma => 9,
            NounState::solid_and_gas => 10,
            NounState::solid_gas_and_plasma => 11,
            NounState::solid_and_liquid => 12,
            NounState::solid_liquid_and_plasma => 13,
            NounState::solid_liquid_and_gas => 14,
            NounState::solid_liquid_gas_and_plasma => 15,
        }
    }

    /// Converts a `u8` value to a `NounState` variant.
    /// Returns `Ok(NounState)` for values 0 to 15, or `Err(Error)` for invalid values.
    pub fn from_u8(value: u8) -> Result<Self, Error> {
        match value {
            0 => Ok(NounState::none),
            1 => Ok(NounState::plasma_only),
            2 => Ok(NounState::gas_only),
            3 => Ok(NounState::gas_and_plasma),
            4 => Ok(NounState::liquid_only),
            5 => Ok(NounState::liquid_and_plasma),
            6 => Ok(NounState::liquid_and_gas),
            7 => Ok(NounState::liquid_gas_and_plasma),
            8 => Ok(NounState::solid_only),
            9 => Ok(NounState::solid_and_plasma),
            10 => Ok(NounState::solid_and_gas),
            11 => Ok(NounState::solid_gas_and_plasma),
            12 => Ok(NounState::solid_and_liquid),
            13 => Ok(NounState::solid_liquid_and_plasma),
            14 => Ok(NounState::solid_liquid_and_gas),
            15 => Ok(NounState::solid_liquid_gas_and_plasma),
            _ => Err(Error::Generic(format!("Invalid noun state u8, {}", value))),
        }
    }

    /// Converts a `NounState` variant to a vector of individual `MatterState` values.
    /// Returns an empty vector for `none`, or the applicable states (solid, liquid, gas, plasma).
    pub fn to_vec(&self) -> Vec<MatterState> {
        match self {
            NounState::none => vec![],
            NounState::plasma_only => vec![MatterState::Plasma],
            NounState::gas_only => vec![MatterState::Gas],
            NounState::gas_and_plasma => vec![MatterState::Gas, MatterState::Plasma],
            NounState::liquid_only => vec![MatterState::Liquid],
            NounState::liquid_and_plasma => vec![MatterState::Liquid, MatterState::Plasma],
            NounState::liquid_and_gas => vec![MatterState::Liquid, MatterState::Gas],
            NounState::liquid_gas_and_plasma => vec![MatterState::Liquid, MatterState::Gas, MatterState::Plasma],
            NounState::solid_only => vec![MatterState::Solid],
            NounState::solid_and_plasma => vec![MatterState::Solid, MatterState::Plasma],
            NounState::solid_and_gas => vec![MatterState::Solid, MatterState::Gas],
            NounState::solid_gas_and_plasma => vec![MatterState::Solid, MatterState::Gas, MatterState::Plasma],
            NounState::solid_and_liquid => vec![MatterState::Solid, MatterState::Liquid],
            NounState::solid_liquid_and_plasma => vec![MatterState::Solid, MatterState::Liquid, MatterState::Plasma],
            NounState::solid_liquid_and_gas => vec![MatterState::Solid, MatterState::Liquid, MatterState::Gas],
            NounState::solid_liquid_gas_and_plasma => vec![MatterState::Solid, MatterState::Liquid, MatterState::Gas, MatterState::Plasma],
        }
    }

    /// Converts a vector of `MatterState` values to a `NounState` variant.
    /// Returns `Ok(NounState)` if the combination is valid, or `Err(Error)` for invalid combinations.
    pub fn from_vec(states: &[MatterState]) -> Result<Self, Error> {
        let mut has_solid = false;
        let mut has_liquid = false;
        let mut has_gas = false;
        let mut has_plasma = false;

        for state in states {
            match state {
                MatterState::Solid => has_solid = true,
                MatterState::Liquid => has_liquid = true,
                MatterState::Gas => has_gas = true,
                MatterState::Plasma => has_plasma = true,
            }
        }

        match (has_solid, has_liquid, has_gas, has_plasma) {
            (false, false, false, false) => Ok(NounState::none),
            (false, false, false, true) => Ok(NounState::plasma_only),
            (false, false, true, false) => Ok(NounState::gas_only),
            (false, false, true, true) => Ok(NounState::gas_and_plasma),
            (false, true, false, false) => Ok(NounState::liquid_only),
            (false, true, false, true) => Ok(NounState::liquid_and_plasma),
            (false, true, true, false) => Ok(NounState::liquid_and_gas),
            (false, true, true, true) => Ok(NounState::liquid_gas_and_plasma),
            (true, false, false, false) => Ok(NounState::solid_only),
            (true, false, false, true) => Ok(NounState::solid_and_plasma),
            (true, false, true, false) => Ok(NounState::solid_and_gas),
            (true, false, true, true) => Ok(NounState::solid_gas_and_plasma),
            (true, true, false, false) => Ok(NounState::solid_and_liquid),
            (true, true, false, true) => Ok(NounState::solid_liquid_and_plasma),
            (true, true, true, false) => Ok(NounState::solid_liquid_and_gas),
            (true, true, true, true) => Ok(NounState::solid_liquid_gas_and_plasma),
        }
    }
}


