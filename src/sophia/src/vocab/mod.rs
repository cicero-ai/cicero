// Copyright 2025 Aquila Labs of Alberta, Canada <matt@cicero.sh>
// Licensed under the Functional Source License, Version 1.1 (FSL-1.1)
// See the full license at: https://cicero.sh/license.txt
// Distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND.

pub use self::cache::VocabCache;
pub use self::category::{VocabCategory, VocabCategoryDatabase, VocabCategoryIndex};
pub use self::database::{
    VocabDatabase, VocabDatabaseMeta, VocabPreProcessDatabase, VocabWordDatabase,
};
pub use self::future_verbs::FutureVerbPhrases;
pub use self::mwe::{Capitalization, MWEType, VocabMWE};
pub use self::phrase_intents::{PhraseIntent, PhraseIntents};
pub use self::pronoun::{Pronoun, PronounCategory, PronounGender, PronounNumber, PronounPerson};
pub use self::spell_check::{
    SpellChecker, SpellCheckerCohort, SpellCheckerCohortPOS, SpellCheckerCohortSize,
};
pub use self::stats::VocabStats;

mod cache;
mod category;
mod database;
pub mod f8;
mod future_verbs;
pub mod mwe;
mod phrase_intents;
mod pronoun;
mod spell_check;
mod stats;
