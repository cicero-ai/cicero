
use serde_derive::{Serialize, Deserialize};
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub city: String,
    pub region: String,
    pub country: String
}

impl Location {
    pub fn new(city: &str, region: &str, country: &str) -> Self {
        Self { 
            city: city.to_string(), 
            region: region.to_string(), 
            country: country.to_string()
        }
    }
}


impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.city, self.region, self.country)
    }
}

impl Hash for Location {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.city.hash(state);
        self.region.hash(state);
        self.country.hash(state);
    }
}


