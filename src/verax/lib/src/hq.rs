
use atlas_http::{HttpBody, HttpRequest, HttpClient};
use serde::de::DeserializeOwned;
use serde::{Serialize, Deserialize};

static HQ_BASE_URL: &str = "https://cicero.sh/api/cicero";
static HQ_PROVIDER_KEY: &str = "bdf25f4cea5b6cf211179368cf75bcb59383";

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
    pub fn send<T: DeserializeOwned>(&self, path: &str, data: &str) -> T {

        // Set headers
        let provider_key = format!("API-Provider-Key: {}", HQ_PROVIDER_KEY);
        let headers = vec![
            &provider_key,
            "Content-Type: application/x-www-form-urlencoded"
        ];

        // Create http request
        let url = format!("{}/{}", HQ_BASE_URL, path);
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





