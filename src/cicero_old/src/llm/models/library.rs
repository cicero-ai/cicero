
use serde_derive::{Serialize, Deserialize};
use std::collections::HashMap;
use std::path::Path;
use std::fs;
use crate::utils::sys;
use log::error;
use super::Model;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelLibrary {
    pub models: HashMap<String, Model>
}


impl ModelLibrary {

    pub fn new() -> Self {

        // Check if file exists
        let lib_file = format!("{}/config/models.yml", sys::get_datadir());
        if !Path::new(&lib_file).exists() {
            return Self { models: HashMap::new() };
        }

        // Load file
        let yaml_str = fs::read_to_string(&lib_file).unwrap();
        let library: ModelLibrary = serde_yaml::from_str(&yaml_str).unwrap();

        library
    }

    /// Add multiple models to the library
    pub fn add_many(&mut self, models: &Vec<Model>) {
        for model in models {
            self.models.insert(model.name.clone(), model.clone());
        }
    }

    /// Save library
    pub fn  save(&mut self) {
        let filename = format!("{}/config/models.yml", sys::get_datadir());
        sys::prepare_parent_dir(&filename);
        let yaml_str = serde_yaml::to_string(&self).unwrap();
        fs::write(filename, yaml_str).unwrap();
    }

    /// Load model config
    pub fn load(&self, name: &str) -> Model {
        let model = match self.models.get(name) {
            Some(r) => r,
            None => {
                error!("Model does not exist within library with name: {}", name);
                std::process::exit(1);
            }
        };
        model.clone()
    }

}

