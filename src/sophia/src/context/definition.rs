
use serde::{Serialize, Deserialize};
use crate::pos_tagger::POSTag;
use crate::error::Error;

pub trait DefinitionTrait {
    fn get_scores_int(&self) -> Vec<u8>;
}

pub trait DefinitionCategory: Default {
    fn to_u8(&self) -> u8;
    fn from_u8(value: u8) -> Result<Self, Error>;
    fn to_string(&self) -> String;
    fn from_str(value: &str) -> Result<Self, Error>;
    fn get_schema(&self) -> String;
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Definition<T: DefinitionCategory> {
    #[serde(skip)]
    pub index: i32,
    #[serde(skip)]
    pub pos: POSTag,
    pub category: T,
    pub definition: String,
    pub scores: Vec<u8>
}

impl<T: DefinitionCategory> Definition<T> {
    pub fn new(category: T, scores: Vec<u8>) -> Self {
        Self {
            category,
            scores,
            ..Default::default()
        }
    }
}


