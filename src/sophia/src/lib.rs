// Copyright 2025 Aquila Labs of Alberta, Canada <matt@cicero.sh>
// Licensed under the PolyForm Noncommercial License 1.0.0
// Commercial use requires a separate license: https://cicero.sh/sophia/
// License text: https://polyformproject.org/licenses/noncommercial/1.0.0/
// Distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND.

#![allow(non_camel_case_types)]

pub use self::error::Error;
pub use self::sophia::Sophia;

pub mod error;
pub mod interpret;
pub mod pos_tagger;
pub mod sophia;
pub mod tokenizer;
pub mod vocab;
