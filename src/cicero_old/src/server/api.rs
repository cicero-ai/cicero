
use atlas_http::HttpResponse;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub status: String,
    pub message: String,
    pub data: T
}

/// Give response
pub fn response<T: Serialize>(status_code: u16, message: &str, data: T) -> HttpResponse {

    // set response
    let response = ApiResponse {
        status: if status_code == 200 { "ok".to_string() } else { "error".to_string() },
        message: message.to_string(),
        data
    };
    let json: String = serde_json::to_string(&response).unwrap();

    HttpResponse::new(&status_code, &vec!["Content-type: application/json".to_string()], &json)
}


/// Check if any fields missing
pub fn check_required(params: &HashMap<String, String>, required: Vec<&str>) -> Option<String> {

    for key in required.iter() {
        if !params.contains_key(key.clone()) {
            return Some(key.to_string())
        }
    }
    None
}

