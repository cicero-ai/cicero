
use rust_bert::resources::ResourceProvider;
use rust_bert::pipelines::common::{ModelResource};
use rust_bert::resources::RemoteResource;
use serde_derive::{Serialize, Deserialize};
use super::{ModelType, ModelSize, ModelSource};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    pub name: String,
    pub description: String,
    pub repo_url: String,
    pub size: ModelSize,
    pub model_type: ModelType,
    pub source: ModelSource,
    pub language: String,
    pub file_config: String,
    pub file_vocab: String,
    pub file_merges: String,
    pub file_model: String,
    pub file_modules: String,
    pub file_pooling_config: String,
    pub file_dense_config: String,
    pub file_dense_weights: String,
    pub file_sentences_config: String,
    pub file_tokenizer_config: String,
}

impl Model {

    /// Get config
    pub fn get_config(&self) -> Box<dyn ResourceProvider + Send> {
        let (name, url) = self.get_tuple("config", &self.file_config);
        Box::new(RemoteResource::from_pretrained(
            (name.as_str(), url.as_str())
        ))
    }

    /// Get vocab
    pub fn get_vocab(&self) -> Box<dyn ResourceProvider + Send> {
        let (name, url) = self.get_tuple("vocab", &self.file_vocab);
        Box::new(RemoteResource::from_pretrained(
            (name.as_str(), url.as_str())
        ))
    }

    /// Get merges
    pub fn get_merges(&self) -> Box<dyn ResourceProvider + Send> {
        let (name, url) = self.get_tuple("merges", &self.file_merges);
        Box::new(RemoteResource::from_pretrained(
            (name.as_str(), url.as_str())
        ))
    }

    /// Get model
    pub fn get_model(&self) -> Box<RemoteResource> {
        let (name, url) = self.get_tuple("model", &self.file_model);
        Box::new(RemoteResource::from_pretrained(
            (name.as_str(), url.as_str())
        ))
    }

    /// Get modules.json
    pub fn get_modules(&self) -> Box<dyn ResourceProvider + Send> {
        let (name, url) = self.get_tuple("modules", &self.file_modules);
        Box::new(RemoteResource::from_pretrained(
            (name.as_str(), url.as_str())
        ))
    }

    /// Get pooling config
    pub fn get_pooling_config(&self) -> Box<dyn ResourceProvider + Send> {
        let (name, url) = self.get_tuple("poolling_config", &self.file_pooling_config);
        Box::new(RemoteResource::from_pretrained(
            (name.as_str(), url.as_str())
        ))
    }

    /// Get demse cpmfog
    pub fn get_dense_config(&self) -> Box<dyn ResourceProvider + Send> {
        let (name, url) = self.get_tuple("dense-config", &self.file_dense_config);
        Box::new(RemoteResource::from_pretrained(
            (name.as_str(), url.as_str())
        ))
    }

    /// Get dense weight
    pub fn get_dense_weights(&self) -> Box<dyn ResourceProvider + Send> {
        let (name, url) = self.get_tuple("dense-weight", &self.file_dense_weights);
        Box::new(RemoteResource::from_pretrained(
            (name.as_str(), url.as_str())
        ))
    }

    /// Get sentences config
    pub fn get_sentences_config(&self) -> Box<dyn ResourceProvider + Send> {
        let (name, url) = self.get_tuple("sbert-config", &self.file_sentences_config);
        Box::new(RemoteResource::from_pretrained(
            (name.as_str(), url.as_str())
        ))
    }

    /// Get tokenizer config
    pub fn get_tokenizer_config(&self) -> Box<dyn ResourceProvider + Send> {
        let (name, url) = self.get_tuple("tokenizer-config", &self.file_tokenizer_config);
        Box::new(RemoteResource::from_pretrained(
            (name.as_str(), url.as_str())
        ))
    }

    /// Get tuple for rust-bert crate
    fn get_tuple(&self, file_type: &str, filename: &String) -> (String, String) {
        let name = format!("{}/{}", self.name, file_type);

        let url = if self.source == ModelSource::HuggingFace {
            format!("{}/resolve/main/{}", self.repo_url.trim_end_matches("/"), filename)
        } else {
            format!("{}/{}", self.repo_url.trim_end_matches("/"), filename)
        };

        (name, url)
    }

}




