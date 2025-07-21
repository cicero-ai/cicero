
use serde_derive::{Serialize, Deserialize};


pub trait CiceroTask {
    fn execute(&self);
}

