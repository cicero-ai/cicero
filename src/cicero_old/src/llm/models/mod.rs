
pub use rust_bert::pipelines::common::ModelType;
pub use self::library::ModelLibrary;
pub use self::model::Model;
use serde_derive::{Serialize, Deserialize};

pub mod model;
pub mod library;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ModelSize {
    Tiny,
    Small,
    Medium,
    Large,
    ExtraLarge
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ModelSource {
    HuggingFace,
    Cicero
}



