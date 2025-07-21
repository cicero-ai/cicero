
use serde_derive::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserResponse {
    pub uuid: Uuid,
    pub apollo_api_key: String,
    pub cicero_public_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextGenerationRequest {
    pub input: Vec<String>,
    pub prefix: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionAnswerRequest {
    pub input: Vec<String>,
    pub context: String
}


