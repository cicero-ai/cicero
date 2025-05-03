pub use self::pos_tag::POSTag;
pub use self::schema::{
    POSTagger, POSTaggerBigramScores, POSTaggerExactMatchTrie, POSTaggerLayer, POSTaggerScores,
    Score,
};

mod pos_tag;
mod schema;
mod tagger;
