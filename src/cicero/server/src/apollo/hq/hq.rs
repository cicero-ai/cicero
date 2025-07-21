

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use atlas_http::HttpClient;
use uuid::Uuid;
use cicero::preludes::ApiResponse;
use crate::llm::models::ModelSize;
use log::error;

const HQ_BASE_URL: &'static str = "http://127.0.0.1/api/cicero";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DefaultModelsResponse {
    pub size: String,
    pub conversation_models: Vec<ConversationModel>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationModel {
    pub size: String,
    pub provider: String,
    pub alias: String,
    pub name: String
}

pub struct CiceroHQ { }

impl CiceroHQ {

    pub fn new() -> Self {
        Self { }
    }

    /// Get default models
    pub fn get_default_models(&self, size: &ModelSize, uuid: &Uuid) -> Vec<ConversationModel> {

        // Initialize
        let size_str = size.to_string();
        let url = format!("{}/models/get_defaults?size={}&uuid={}", HQ_BASE_URL, size.to_string(), uuid.to_string());
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

        json.data.conversation_models.clone()
    }

}



