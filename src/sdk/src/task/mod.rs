
use serde_derive::{Serialize, Deserialize};
pub use self::kit::CiceroTask;
pub use self::iterate::IterateTask;

pub mod factory;
pub mod kit;
pub mod iterate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskName {
    none,
    Iterate(IterateTask)
}

pub struct Task {
    task: TaskName,
    kit: Box<dyn CiceroTask>
}



