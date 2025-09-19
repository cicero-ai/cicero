// Copyright 2025 Aquila Labs of Alberta, Canada <matt@cicero.sh>
// Licensed under the PolyForm Noncommercial License 1.0.0
// Commercial use requires a separate license: https://cicero.sh/sophia/
// License text: https://polyformproject.org/licenses/noncommercial/1.0.0/
// Distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND.

pub use self::antecedent_buffer::AntecedentBuffer;
pub use self::buffer::Buffer;
pub use self::coref_categories::CoreferenceCategories;
pub use self::interpretation::Interpretation;
pub use self::interpreter::Interpreter;
pub use self::phrase::{
    Adjective, Adverb, Noun, NounModifier, NounOwner, NounSibling, Phrase, PhraseTense, Verb,
    VerbModifier, VerbSibling,
};
pub use self::phrase_buffer::PhraseBuffer;

mod antecedent_buffer;
mod buffer;
mod coref_categories;
mod interpretation;
mod interpreter;
mod phrase;
mod phrase_buffer;
