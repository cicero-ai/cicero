
pub enum IterateTaskCategory {
    Database,
    KvStore,
    Directory,
    EmailInbox
}

pub struct IterateTaskFactory { }

impl IterateTaskFactory {

    /// Create new instance of factory
    pub fn new() -> Self {
        Self { }
    }

    /// Try to coerce a phrase into an actionable task
    pub fn try_into(phrase: &String) {

    }

}





