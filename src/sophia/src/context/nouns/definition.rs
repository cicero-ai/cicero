
use crate::context::{Definition, DefinitionTrait};
use super::{NounState, NounIndustry, NounCategory};

impl DefinitionTrait for Definition<NounCategory> {
    /// Returns the integer scores (State of Matter and Industry) for tangible nouns.
    /// For non-tangible nouns (abstract, location, event), returns an empty vector.
    fn get_scores_int(&self) -> Vec<u8> {
        vec![]
    }
}

impl Definition<NounCategory> {
    /// Extracts the primary state of matter from the left 4 bits of scores_int[0].
    /// Returns NounState::none if scores_int is empty or invalid.
    pub fn get_primary_state(&self) -> NounState {
        if self.scores.is_empty() {
            return NounState::none;
        }
        NounState::from_u8(self.scores[0]).unwrap_or(NounState::none)
    }

    /// Returns a vector of NounState variants based on the bit mask (solid, liquid, gas, plasma).
    pub fn get_possible_states(&self) -> Vec<NounState> {
        if self.scores.is_empty() {
            return vec![];
        }
        let possible_bits = self.scores[1] & 0x0F;
        let mut states = vec![];
        if possible_bits & 0b1000 != 0 {
            states.push(NounState::solid_only);
        }
        if possible_bits & 0b0100 != 0 {
            states.push(NounState::liquid_only);
        }
        if possible_bits & 0b0010 != 0 {
            states.push(NounState::gas_only);
        }
        if possible_bits & 0b0001 != 0 {
            states.push(NounState::plasma_only);
        }
        states
    }

    /// Extracts the 32-bit industry mask from scores_int[1..=4].
    /// Returns a vector of NounIndustry variants for each bit set to 1.
    /// Returns an empty vector if scores_int has fewer than 5 elements.
    pub fn get_industries(&self) -> Vec<NounIndustry> {
        if self.scores.len() < 5 {
            return vec![];
        }
        let mut industries = vec![];
        let mask = u32::from_be_bytes([
            self.scores[1],
            self.scores[2],
            self.scores[3],
            self.scores[4],
        ]);
        for i in 0..32 {
            if (mask >> (31 - i)) & 1 == 1 {
                if let Ok(industry) = NounIndustry::from_u8(i as u8) {
                    industries.push(industry);
                }
            }
        }
        industries
    }

    /// Extracts the top three industries from the 16-bit value in scores_int[5..=6].
    /// Returns a vector of up to three NounIndustry variants, in order, if they exist.
    /// Returns an empty vector if scores_int has fewer than 7 elements or if values are invalid.
    pub fn get_top_industries(&self) -> Vec<NounIndustry> {
        if self.scores.len() < 7 {
            return vec![];
        }
        let top_bits = u16::from_be_bytes([self.scores[5], self.scores[6]]);
        let mut industries = vec![];
        // Extract 3x 5-bit values (15 bits total, ignoring the last bit)
        let first = ((top_bits >> 10) & 0x1F) as u8;
        let second = ((top_bits >> 5) & 0x1F) as u8;
        let third = (top_bits & 0x1F) as u8;
        for value in [first, second, third] {
            if let Ok(industry) = NounIndustry::from_u8(value) {
                if value < 32 {
                    industries.push(industry);
                }
            }
        }
        industries
    }
}


