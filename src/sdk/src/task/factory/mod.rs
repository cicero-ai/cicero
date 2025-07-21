
pub use self::iterate::IterateTaskFactory;

pub mod iterate;

pub struct TaskFactory { }

impl TaskFactory {

    /// Iterate over a set of items
    pub fn iterate() -> IterateTaskFactory {
        IterateTaskFactory::new()
    }


}

