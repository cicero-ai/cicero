
use atlas_http::HttpClient;
use std::collections::HashMap;
use serde_derive::{Serialize, Deserialize};
use uuid::Uuid;
use crate::llm::models::{Model, ModelSize, ModelLibrary};
use crate::server::config::{CiceroServerConfig, ConfigModels};
use crate::server::api::ApiResponse;
use log::error;

const HQ_BASE_URL: &'static str = "http://127.0.0.1:8901";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultModelsResponse {
    pub size: String,
    pub defaults: ConfigModels,
    pub conversation_models: Vec<ConversationModel>,
    pub models: Vec<Model>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationModel {
    pub is_default: u8,
    pub size: String,
    pub alias: String,
    pub name: String
}

pub struct CiceroHQ { }



impl CiceroHQ {

    pub fn new() -> Self {
        Self { }
    }

    /// Get default models
    pub fn get_default_models(&self, size: &ModelSize, uuid: &Uuid) -> (CiceroServerConfig, Vec<ConversationModel>) {

        // Initialize
        let size_str = serde_json::to_string(&size).unwrap().to_lowercase().trim_end_matches('"').trim_start_matches('"').to_string();
        let url = format!("{}/api/cicero/models/get_defaults?size={}&uuid={}", HQ_BASE_URL, size_str, uuid.to_string());
        let mut http = HttpClient::builder().build_sync();

        // Get default models
        let res = match http.get(&url) {
            Ok(r) => r,
            Err(e) => {
                error!("Unable to retrieve list of default models for size {}, error: {}", size_str, e.to_string());
                std::process::exit(1);
            }
        };

        // Deserialize response
        let json: ApiResponse::<DefaultModelsResponse> = match serde_json::from_str(&res.body()) {
            Ok(r) => r,
            Err(e) => {
                error!("Unable to deserialize default models response from HQ, error: {}", e);
                std::process::exit(1);
            }
        };

        // Get config
        let mut config = CiceroServerConfig::default();
        config.models = json.data.defaults.clone();

        // Save models to library
        let mut library = ModelLibrary::new();
        library.add_many(&json.data.models);
        library.save();

        (config, json.data.conversation_models.clone()) 
    }

}


