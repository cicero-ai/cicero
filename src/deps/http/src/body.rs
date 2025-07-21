
use crate::error::Error;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::Path;
use urlencoding::{decode, encode};

#[derive(Clone, Debug)]
pub struct HttpBody {
    is_form_post: bool,
    params: HashMap<String, String>,
    raw: Vec<u8>,
    boundary: String,
    files: HashMap<String, String>,
}


impl HttpBody {
    // Instantiate new body
    pub fn new(params: &HashMap<String, String>, raw: &[u8]) -> Self {
        let boundary: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(|c| c as char)
            .collect();

        Self {
            is_form_post: params.keys().len() > 0 || raw.len() > 0,
            params: params.clone(),
            raw: raw.clone().to_vec(),
            boundary,
            files: HashMap::new(),
        }
    }

    /// Instantiate an empty body.
    pub fn empty() -> Self {
        Self::new(&HashMap::new(), &Vec::new())
    }

    /// Generate body from str
    pub fn from_string(data: &str) -> Self {
        // Create pairs
        let mut params: HashMap<String, String> = HashMap::new();
        for pair in data.split('&') {
            if let Some(index) = pair.find('=') {
                params.insert(
                    pair[..index].to_string(),
                    decode(pair[index + 1..].trim()).unwrap().to_string(),
                );
            }
        }

        Self::new(&params, &Vec::new())
    }

    /// Generate body from hashmap
    pub fn from_map(params: &HashMap<&str, &str>) -> Self {
        let formatted_params = params
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect();
        Self::new(&formatted_params, &Vec::new())
    }

    // Generate body with raw vec<u8> (eg. JSON object).  Body is not split and formatted into post params.
    pub fn from_raw(data: &[u8]) -> Self {
        Self::new(&HashMap::new(), data)
    }

    // Generate body with raw str (eg. JSON object).  Body is not split and formatted into post params.
    pub fn from_raw_str(data: &str) -> Self {
        Self::new(&HashMap::new(), data.as_bytes())
    }

    /// Add post parameter
    pub fn set_param(&mut self, key: &str, value: &str) {
        *self
            .params
            .entry(key.to_string())
            .or_insert(value.to_string()) = value.to_string();
        self.is_form_post = true;
    }

    // Upload a file
    pub fn upload_file(&mut self, param_name: &str, file_path: &str) -> Result<(), Error> {
        // Ensure file exists
        if !Path::new(&file_path).exists() {
            return Err(Error::FileNotExists(file_path.to_string()));
        }
        *self
            .files
            .entry(param_name.to_string())
            .or_insert(file_path.to_string()) = file_path.to_string();

        Ok(())
    }

    /// Format body for HTTP message
    pub fn format(&self) -> Vec<u8> {
        if !self.files.is_empty() {
            return self.format_multipart();
        } else if self.raw.len() > 0 {
            return self.raw.clone();
        } else if !self.is_form_post {
            return Vec::new();
        }

        let body = self
            .params
            .iter()
            .map(|(key, value)| format!("{}={}", key, encode(value)))
            .collect::<Vec<String>>()
            .join("&");

        body.as_bytes().to_vec()
    }

    /// Format multipart message, used for uploading files
    fn format_multipart(&self) -> Vec<u8> {

        // Go through params
        let mut body: Vec<u8> = Vec::new();
        for (key, value) in self.params.iter() {
            let section = format!(
                "--{}\r\nContent-Disposition: form-data; name=\"{}\"\r\n\r\n{}\r\n",
                self.boundary, key, value
            );
            body.extend_from_slice(section.as_bytes());
        }

        // Go through files
        for (key, filepath) in self.files.iter() {
            let (filename, mime_type, contents) = self.get_file_info(filepath);
            let section = format!("--{}\r\nContent-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\nContent-Type: {}\r\n\r\n", self.boundary, key, filename, mime_type);
            body.extend_from_slice(section.as_bytes());
            body.extend_from_slice(&contents);
            body.extend_from_slice("\r\n".as_bytes());
        }
        body.extend_from_slice(format!("--{}--\r\n", self.boundary).as_bytes());

        body
    }

    /// Get info for uploaded file
    fn get_file_info(&self, filepath: &String) -> (String, String, Vec<u8>) {
        // Get filename
        let pos = filepath
            .rfind('/')
            .or_else(|| filepath.rfind('\\'))
            .unwrap();
        let filename = filepath[pos + 1..].to_string();

        // Get mime type
        let mime_guess = mime_guess::from_path(filepath);
        let mime_type = if mime_guess.count() > 0 {
            mime_guess.first().unwrap().to_string()
        } else {
            "application/octet-stream".to_string()
        };

        let _file = File::open(filepath).unwrap();
        let content =
            fs::read(filepath).unwrap_or_else(|_| panic!("Unable to read file at, {}", filepath));

        (filename, mime_type, content)
    }
    /// Get is_form_post
    pub fn is_form_post(&self) -> bool {
        self.is_form_post
    }

    /// Get params
    pub fn params(&self) -> HashMap<String, String> {
        self.params.clone()
    }

    /// Add param
    pub fn add_param(&mut self, key: &str, value: &str) {
        *self.params.entry(key.to_string()).or_default() = value.to_string();
        self.is_form_post = true;
    }

    /// Get raw data
    pub fn get_raw(&self) -> Vec<u8> {
        self.raw.clone()
    }

    /// Get boundary
    pub fn boundary(&self) -> String {
        self.boundary.clone()
    }

    /// Get uploaded files
    pub fn files(&self) -> HashMap<String, String> {
        self.files.clone()
    }
}
