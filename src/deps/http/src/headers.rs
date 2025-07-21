
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct HttpHeaders {
    pairs: HashMap<String, Vec<String>>,
    lower_map: HashMap<String, String>,
}


impl HttpHeaders {
    /// Generate blank instance of struct.
    pub fn new() -> Self {
        Self {
            pairs: HashMap::new(),
            lower_map: HashMap::new(),
        }
    }

    /// Parse lines from HTTP response into struct.
    pub fn from_vec(header_lines: &Vec<String>) -> Self {
        // Initialize
        let mut pairs: HashMap<String, Vec<String>> = HashMap::new();
        let mut lower_map: HashMap<String, String> = HashMap::new();

        // GO through lines
        for line in header_lines {
            if let Some(cindex) = line.find(':') {
                let key = line[..cindex].to_string();
                pairs
                    .entry(key.clone())
                    .or_default()
                    .push(line[cindex + 1..].trim().to_string());
                lower_map.insert(key.to_lowercase(), key);
            }
        }

        Self { pairs, lower_map }
    }

    /// Create headers instance from hashmap
    pub fn from_hash(headers: &HashMap<&str, &str>) -> Self {
        let pairs: HashMap<String, Vec<String>> = headers
            .iter()
            .map(|(k, v)| {
                let value: Vec<String> = v.split(';').map(|e| e.trim().to_string()).collect();
                (k.to_string(), value)
            })
            .collect();

        let lower_map: HashMap<String, String> = headers
            .keys()
            .map(|k| (k.to_lowercase().to_string(), k.to_string()))
            .collect();

        Self { pairs, lower_map }
    }

    // Check whether or not header exists, case-sensitive
    pub fn has(&self, key: &str) -> bool {
        self.pairs.contains_key(&key.to_string())
    }

    // Check whether or not header exists, case-insensitive
    pub fn has_lower(&self, key: &str) -> bool {
        if let Some(hdr_key) = self.lower_map.get(key.to_lowercase().as_str()) {
            return self.has(hdr_key);
        }
        false
    }

    /// Get value of HTTP header.  Case-sensitive, will
    /// only return first instance if multiple instances of same key exist.
    pub fn get(&self, key: &str) -> Option<String> {
        self.pairs.get(key).and_then(|val| val.get(0)).cloned()
    }

    /// Get value of HTTP header.  Same as get(), but case-insensitive.
    pub fn get_lower(&self, key: &str) -> Option<String> {
        if let Some(hdr_key) = self.lower_map.get(key.to_lowercase().as_str()) {
            return self.get(hdr_key);
        }
        None
    }

    /// Get vector of all values of header, case-sensitive.
    pub fn get_vec(&self, key: &str) -> Vec<String> {
        if let Some(values) = self.pairs.get(key) {
            return values.clone();
        }
        vec![]
    }

    // Get vector of all values of header, case-insensitive.
    pub fn get_lower_vec(&self, key: &str) -> Vec<String> {
        if let Some(hdr_key) = self.lower_map.get(key.to_lowercase().as_str()) {
            return self.get_vec(hdr_key);
        }
        vec![]
    }

    /// Get header line, all values delimited by ";", case-sensitive.
    pub fn get_line(&self, key: &str) -> Option<String> {
        if let Some(val) = self.pairs.get(key) {
            return Some(val.join("; "));
        }
        None
    }

    /// Get header line, all values delimited by ";", case-sensitive.
    pub fn get_lower_line(&self, key: &str) -> Option<String> {
        if let Some(hdr_key) = self.lower_map.get(key.to_lowercase().as_str()) {
            return self.get_line(hdr_key);
        }
        None
    }

    /// Get all headers as hashmap
    pub fn all(&self) -> HashMap<String, Vec<String>> {
        self.pairs.clone()
    }

    /// Set header, replacing any existing header value with same key
    pub fn set(&mut self, key: &str, value: &str) {
        let val = vec![value.to_string()];
        *self.pairs.entry(key.to_string()).or_insert(val) = val.clone();
        *self
            .lower_map
            .entry(key.to_lowercase().to_string())
            .or_insert(key.to_string()) = key.to_string();
    }

    /// Set header, replacing any existing header value with same key
    pub fn set_vec(&mut self, key: &str, value: &Vec<&str>) {
        let val = value.iter().map(|s| s.to_string()).collect();
        *self.pairs.entry(key.to_string()).or_insert(val) = val.clone();
        *self
            .lower_map
            .entry(key.to_lowercase().to_string())
            .or_insert(key.to_string()) = key.to_string();
    }

    /// Add value to existing header, or add new header if key non-existent.
    pub fn add(&mut self, key: &str, value: &str) {
        self.pairs
            .entry(key.to_string())
            .or_default()
            .push(value.to_string());
        *self
            .lower_map
            .entry(key.to_lowercase().to_string())
            .or_insert(key.to_string()) = key.to_string();
    }

    /// Delete header
    pub fn delete(&mut self, key: &str) {
        self.lower_map.remove(&key.to_lowercase().to_string());
        self.pairs.remove(&key.to_string());
    }

    /// Clear / purge all headers
    pub fn clear(&mut self) {
        self.pairs.clear();
        self.lower_map.clear();
    }
}
