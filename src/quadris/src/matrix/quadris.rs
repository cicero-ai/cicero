
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Quadris {
    profile: HashMap<i32, Cylinder>,
    entities: HashMap<i32, Cylinder>
    entity_groups: HashMap<i32, Vec<i32>>,
    traits: HashMap<String, Trait>
}

Likes / Dislikes
Rules and Avoidances
Schedules and Routines



