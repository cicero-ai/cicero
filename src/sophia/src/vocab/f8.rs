// Copyright 2025 Aquila Labs of Alberta, Canada <matt@cicero.sh>
// Licensed under the PolyForm Noncommercial License 1.0.0
// Commercial use requires a separate license: https://cicero.sh/sophia/
// License text: https://polyformproject.org/licenses/noncommercial/1.0.0/
// Distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND.

use serde::{Deserialize, Serialize};
use std::convert::From;
use std::fmt;
use std::ops::{Add, AddAssign, Mul};

/// A fixed-point 8-bit representation for floating-point values in the range [0.0, 1.0], storing a sum and value history.
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct f8 {
    values: Vec<u8>,
    pub sum: u8,
}

impl f8 {
    /// Creates a new f8 instance from a u8 value, initializing the sum and value history.
    pub fn new(value: u8) -> Self {
        f8 {
            values: vec![value],
            sum: value,
        }
    }

    /// Converts the f8 value to a f32 in the range [0.0, 1.0].
    pub fn to_f32(&self) -> f32 {
        self.sum as f32 / 255.0
    }

    /// Calculates the sum of all values in the f8 instance as a u16.
    fn calculate_sum(&self) -> u16 {
        self.values.iter().map(|&v| v as u16).sum()
    }

    /// Returns the sum as a u8, representing the quantized value.
    fn to_u8(&self) -> u8 {
        self.sum
    }

}

impl From<f32> for f8 {
    /// Converts a f32 value in [0.0, 1.0] to an f8 instance, quantizing to a u8.
    fn from(value: f32) -> Self {
        // Map the range [0.0, 1.0] to [0, 255]
        let quantized = (value * 255.0).round() as u8;
        f8::new(quantized)
    }
}

impl From<f8> for f32 {
    /// Converts an f8 instance to a f32 value in [0.0, 1.0].
    fn from(val: f8) -> Self {
        (val.to_u8() as f32) / 255.0
    }
}

impl Add for f8 {
    /// Adds two f8 instances, combining their value histories and capping the sum at 255.
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        self.values.extend(other.values);
        let new_sum = self.calculate_sum().min(255) as u8;
        f8 {
            values: self.values,
            sum: new_sum,
        }
    }
}

impl AddAssign<usize> for f8 {
    fn add_assign(&mut self, rhs: usize) {
        self.values.push(rhs as u8);
        self.sum += rhs as u8;
    }
}

impl Mul<f32> for f8 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        // Convert the F8 value to f32, multiply, then convert back to F8
        let result_f32 = (self.to_u8() as f32 / 255.0) * rhs;
        f8::from(result_f32)
    }
}

impl Serialize for f8 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(self.sum)
    }
}

impl<'de> Deserialize<'de> for f8 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        Ok(f8::new(value))
    }
}

impl PartialOrd for f8 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.calculate_sum().cmp(&other.calculate_sum()))
    }
}

impl Ord for f8 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.calculate_sum().cmp(&other.calculate_sum())
    }
}

impl fmt::Display for f8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_f32())
    }
}
