
use atlas_http::{HttpClient, HttpRequest, HttpBody};
use std::collections::HashMap;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::server::api::ApiResponse;
use crate::utils::sys;
use log::{error, debug};
use crate::error::Error;
use std::fs;
use crate::CLIENT_CONFIG;

// Send JAON to Apollo server
pub fn send_json<T: DeserializeOwned, S: Serialize>(path: &str, method: &str, params: S) -> Result<T, Error> {

    // Create http request
    let json_str = serde_json::to_string(&params).unwrap();
    let req = create_apollo_request(&path, &method, "application/json", &HttpBody::from_raw(&json_str.as_bytes()));
    debug!("Sending request to Apollo client at {}", path);

        // Send request
    let res = send_apollo::<T>(&req)?;
    Ok(res)
}

/// Send params request to Apollo
pub fn send_body<T: DeserializeOwned>(path: &str, method: &str, body: &HttpBody) -> Result<T, Error> {

    // Create http request
    let req = create_apollo_request(&path, &method, "application/x-www-form-urlencoded", &body);

    // Send request
    let res = send_apollo::<T>(&req)?;
    Ok(res)
}

/// Create request destined for Apollo
pub fn create_apollo_request(path: &str, method: &str, content_type: &str, body: &HttpBody) -> HttpRequest {

    // HTTP headers
    let auth_line = format!("Cicero-API-Key: {}", sys::get_apollo_api_key());
    let ctype_line = format!("Content-Type: {}", content_type);
    let http_headers = vec![
        ctype_line.as_str(),
        auth_line.as_str()
    ];

    // Get request
    let url = format!("http://{}:{}/{}", CLIENT_CONFIG.daemons.apollo.0, CLIENT_CONFIG.daemons.apollo.1, path);
    HttpRequest::new(&method, &url, &http_headers, &body)
}

/// Send http request to Apollo server
fn send_apollo<T: DeserializeOwned>(req: &HttpRequest) -> Result<T, Error> {

    // Send http request
    let mut http = HttpClient::builder().build_sync();
    let res = match http.send(&req) {
        Ok(r) => r,
        Err(e) => {
            error!("Unable to send HTTP request to {}, error: {}", req.url, e);
            std::process::exit(1);
        }
    };
    let body = format!("{}", res.body().clone());

    // Check response status
    if res.status_code() != 200 {
        error!("Received a {} status code from server, terminating.\n", res.status_code());
        //error!("Request URL: {}\n\n", url);
        //error!("    REQUEST BODY\n\n{}\n", json_str);
        error!("    RESPONSE BODY\n\n{}\n", res.body());
        std::process::exit(1);
    }

    // Decode json
    let json: ApiResponse::<T> = match serde_json::from_str(body.as_str()) {
        Ok(r) => r,
        Err(e) => return Err( Error::RpcClient(format!("Invalid JSOn response received: {}", body)))
    };

    Ok(json.data)
}



