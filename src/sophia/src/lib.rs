#![allow(non_camel_case_types)]
#![allow(clippy::non_camel_case_types)]

pub use self::error::Error;
pub use self::sophia::Sophia;

pub mod error;
pub mod interpreter;
pub mod pos_tagger;
pub mod sophia;
pub mod tokenizer;
pub mod vocab;
