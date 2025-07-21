
use atlas_http::{HttpBody, HttpRequest, HttpClient};
use serde::de::DeserializeOwned;
use serde::{Serialize, Deserialize};
use crate::CONFIG;

static HQ_BASE_URL: &str = "https://127.0.0.1/api/cicero";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub status: String,
    pub message: String,
    pub data: T
}

pub struct CiceroHQ { }

impl CiceroHQ {

    pub fn new() -> Self {
        Self { }
    }

    // Send http request
    pub fn send<T: DeserializeOwned>(&self, data: &str) -> T {

        // Set variables
        let provider_key = format!("API-Provider-Key: {}", CONFIG.hq_provider_key);
        let api_key = format!("API-Key: {}", CONFIG.hq_api_key);
        let api_secret = format!("API-Secret: {}", CONFIG.hq_api_secret);

        let headers = vec![
            &provider_key,
            &api_key,
            &api_secret,
            "Content-Type: application/x-www-form-urlencoded"
        ];

        // Create http request
        let url = format!("{}/license/create", HQ_BASE_URL);
        let req = HttpRequest::new("POST", &url.as_str(), &headers, &HttpBody::from_string(&data));

        // Send http request
        let mut http = HttpClient::builder().noverify_ssl().build_sync();
        let res = http.send(&req).unwrap();

        // Parse response
        let json: ApiResponse::<T> = match serde_json::from_str(&res.body().as_str()) {
            Ok(r) => r,
            Err(e) => {
                println!("Error, unable to parse JSON response from server:\n\nResponse: {}", res.body());
                std::process::exit(1);
            }
        };

        json.data
    }

}





