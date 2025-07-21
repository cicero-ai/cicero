
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use atlas_http::{HttpRequest, HttpResponse};
use url::Url;
use regex::Regex;
use super::{AuthUser, ApiResponse};
use crate::Error;


pub trait HttpRoute {
    fn process(&self, http_req: &HttpRequest, path_params: &HashMap<String, String>, user: &AuthUser) -> Result<ApiResponse, Error>;
}

#[derive(Default)]
pub struct HttpRouter {
    routes: HashMap<HttpMethod, Route>,
    children: HashMap<String, Box<HttpRouter>>,
}

struct Route {
    method: HttpMethod,
    uri: String,
    exact_uri: bool,
    plugin: String,
    target: Box<dyn HttpRoute>,
}

#[derive(Default, Eq, PartialEq, Hash, Clone)]
pub enum HttpMethod {
    #[default]
    Any,
    Get,
    Post,
    Put,
    Delete,
}

impl HttpMethod {
    fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "get" => HttpMethod::Get,
            "post" => HttpMethod::Post,
            "put" => HttpMethod::Put,
            "delete" => HttpMethod::Delete,
            _ => HttpMethod::Any,
        }
    }

    fn to_string(&self) -> String {
        match self {
            HttpMethod::Any => "ANY".to_string(),
            HttpMethod::Get => "GET".to_string(),
            HttpMethod::Post => "POST".to_string(),
            HttpMethod::Put => "PUT".to_string(),
            HttpMethod::Delete => "DELETE".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse {
    pub status: String,
    pub message: String,
    pub data: String
}

impl HttpRouter {
    pub fn add(&mut self, uri: &str, method: HttpMethod, exact_uri: bool, plugin: &str, target: Box<dyn HttpRoute>) {
        let segments: Vec<&str> = uri.trim_start_matches('/').split('/').collect();
        let mut current = self;

        for segment in segments {
            if segment.is_empty() {
                continue;
            }
            current = current.children
                .entry(segment.to_string())
                .or_insert_with(|| Box::new(HttpRouter::default()));
        }

        current.routes.insert(method.clone(), Route {
            method,
            uri: uri.to_string(),
            exact_uri,
            plugin: plugin.to_string(),
            target,
        });
    }

    pub fn delete(&mut self, uri: &str, method: HttpMethod) -> bool {
        let segments: Vec<&str> = uri.trim_start_matches('/').split('/').collect();
        let mut current = self;

        for segment in segments {
            if segment.is_empty() {
                continue;
            }
            if let Some(child) = current.children.get_mut(segment) {
                current = child;
            } else {
                return false; // Path not found
            }
        }

        current.routes.remove(&method).is_some()
    }

    pub fn delete_plugin(&mut self, plugin: &str) {
        self.children.retain(|key, _| !key.starts_with(plugin));
    }

    pub fn handle(&self, http_req: &HttpRequest, user: &AuthUser) -> Result<HttpResponse, Error> {
        let url = Url::parse(&http_req.url).map_err(|e| Error::Generic(format!("Invalid URL: {}", e)))?;
        let path_segments: Vec<&str> = url.path().trim_start_matches('/').split('/').filter(|s| !s.is_empty()).collect();
        let method = HttpMethod::from_str(&http_req.method);

        let mut current = self;
        let mut path_params = HashMap::new();
        let mut matched_route: Option<&Route> = None;

        // Traverse Trie as far as possible
        for (i, segment) in path_segments.iter().enumerate() {
            if let Some(child) = current.children.get(*segment) {
                current = child;
                if let Some(route) = current.routes.get(&method) {
                    matched_route = Some(route);
                }
            } else {
                // Check for dynamic param (e.g., :slug)
                for (key, child) in &current.children {
                    if key.starts_with(':') {
                        current = child;
                        let param_name = key.trim_start_matches(':');
                        path_params.insert(param_name.to_string(), segment.to_string());
                        if let Some(route) = current.routes.get(&method) {
                            matched_route = Some(route);
                        }
                        break;
                    }
                }
            }

            // Stop if exact match required and path continues
            if let Some(route) = matched_route {
                if route.exact_uri && i < path_segments.len() - 1 {
                    matched_route = None;
                }
            }
        }

        // Process matched route or return error
        match matched_route {
            Some(route) => {
                // Check URI for dynamic params
                let param_regex = Regex::new(r":([^/]+)").unwrap();
                for cap in param_regex.captures_iter(&route.uri) {
                    if let Some(param_name) = cap.get(1) {
                        let param_value = path_segments
                            .get(path_segments.iter().position(|s| *s == param_name.as_str()).unwrap_or(0))
                            .unwrap_or(&"");
                        path_params.insert(param_name.as_str().to_string(), param_value.to_string());
                    }
                }

                match route.target.process(http_req, &path_params, user) {
                    Ok(api_response) => Ok(HttpResponse::new(
                        &200,
                        &vec!["Content-type: application/json".to_string()],
                        &"".to_string()
                    )),
                    Err(e) => Ok(HttpResponse::new(
                        &500,
                        &vec!["Content-type: application/json".to_string()],
                        &format!(r#"{{"status":"500","message":"Internal Server Error: {}"}}"#, e),
                    )),
                }
            }
            None => Ok(HttpResponse::new(
                &404,
                &vec!["Content-type: text/plain".to_string()],
                &"File Not Found".to_string(),
            )),
        }
    }
}

