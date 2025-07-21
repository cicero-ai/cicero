
use std::opts::Range;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct Cylinder {
    range: Range<i32>,
    word_index: i32,
    entity_type: EntityType,
    //faiss: Arc<Mutex<FaissIndex>>,
    dimensions: Vec<Feature>,
    segments: HashMap<i32, Cylinder>
}

enum EntityType {
    personal,
    individual,   // family member, friend, co-worker, business colleague, employee
    group,   // societies, clubs, sports teams, etc.
    energy,    // work, learning, project, home repair, anything involving energy
    other
}

struct Feature {
    range: Range<i32>,
    word_index: i32
}



