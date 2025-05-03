use crate::pos_tagger::POSTag;
use crate::tokenizer::Token;
use crate::vocab::VocabDatabase;
use std::ops::Range;

/// Stores category ranges for coreference resolution, including named entity recognition (NER) and noun-based person and entity classifications.
#[derive(Clone, Default)]
pub struct CoreferenceCategories {
    ner_person: Range<i16>,
    noun_person: Vec<Range<i16>>,
    ner_entity: Vec<Range<i16>>,
    noun_entity: Vec<Range<i16>>,
}

impl CoreferenceCategories {
    /// Creates a new CoreferenceCategories instance, initializing ranges from the provided vocabulary database.
    pub fn new(vocab: &VocabDatabase) -> Self {
        Self {
            ner_person: vocab.categories.ner.path2range("person").unwrap_or(0..0),
            noun_person: Self::compile_noun_person(vocab),
            ner_entity: Self::compile_ner_entity(vocab),
            noun_entity: Self::compile_noun_entity(vocab),
        }
    }

    /// Compiles a list of NER entity category ranges for facilities, organizations, and businesses from the vocabulary database.
    pub fn compile_ner_entity(vocab: &VocabDatabase) -> Vec<Range<i16>> {
        let mut res: Vec<Range<i16>> = Vec::new();
        for label in ["facility", "organization", "business"].iter() {
            if let Some(r) = vocab.categories.ner.path2range(label) {
                res.push(r);
            }
        }

        res
    }

    /// Compiles a list of noun person category ranges for military ranks, family relations, occupations, corporate jobs, and individuals.
    pub fn compile_noun_person(vocab: &VocabDatabase) -> Vec<Range<i16>> {
        // Set paths
        let paths = [
            "military/military_rank",
            "health_and_human/family_relation",
            "education/occupation",
            "business_and_finance/corporate_job",
            "personnel/individual",
        ];

        let mut res: Vec<Range<i16>> = Vec::new();
        for path in paths.iter() {
            if let Some(r) = vocab.categories.nouns.path2range(path) {
                res.push(r);
            }
        }

        res
    }

    /// Compiles a list of noun entity category ranges for transportation, military vehicles, landforms, infrastructure, and groups.
    pub fn compile_noun_entity(vocab: &VocabDatabase) -> Vec<Range<i16>> {
        // Set paths
        let paths = [
            "transportation/aircraft",
            "transportation/automobile",
            "transportation/bycycle",
            "transportation/_public_transportation",
            "transportation/ship",
            "military/vehicle",
            "environment/landform",
            "architecture_and_construction/infrastructure",
            "personnel/group",
        ];

        let mut res: Vec<Range<i16>> = Vec::new();
        for path in paths.iter() {
            if let Some(r) = vocab.categories.nouns.path2range(path) {
                res.push(r);
            }
        }

        res
    }

    /// Checks if a token represents a person, based on NER person range or noun person categories.
    pub fn is_person(&self, token: &Token) -> bool {
        if token.is_named_entity() {
            return token.has_ner(&self.ner_person);
        }

        self.noun_person.iter().any(|r| token.has_category(r))
    }

    /// Checks if a token represents an entity, based on NER entity ranges, plural noun person categories, or noun entity categories.
    pub fn is_entity(&self, token: &Token) -> bool {
        if token.is_named_entity() {
            return self.ner_entity.iter().any(|r| token.has_ner(r));
        }

        if token.pos == POSTag::NNS && self.noun_person.iter().any(|r| token.has_category(r)) {
            return true;
        }

        self.noun_entity.iter().any(|r| token.has_category(r))
    }
}
