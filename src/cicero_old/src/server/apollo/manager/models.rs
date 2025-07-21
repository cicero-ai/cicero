
use crate::llm::models::ModelLibrary;

pub struct ModelManager {
    library: ModelLibrary
}

impl ModelManager {

    pub fn new() -> Self {
        Self {
            library: ModelLibrary::new()
        }
    }

}



