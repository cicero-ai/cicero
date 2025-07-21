
use super::HttpHeaders;
use crate::cookie::Cookie;
use crate::error::{Error, FileNotCreatedError};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use url::Url;

#[derive(Clone, Debug)]
pub struct CookieJar {
    jar_file: String,
    auto_update: bool,
    cookies: HashMap<String, Cookie>,
}

impl CookieJar {
    /// Instantiate a new, empty cookie jar
    pub fn new() -> Self {
        Self {
            jar_file: String::new(),
            auto_update: false,
            cookies: HashMap::new(),
        }
    }

    /// Instantiate a new cookie jar from a Netscape formatted cookies.txt file
    pub fn from_file(jar_file: &str, _auto_update: bool) -> Result<Self, Error> {
        // Check file exists
        if !Path::new(jar_file).exists() {
            return Err(Error::FileNotExists(jar_file.to_string()));
        }

        // Get file contents
        let contents = fs::read_to_string(jar_file).unwrap();
        let mut jar = Self::from_string(&contents);
        jar.jar_file = jar_file.to_string();
        jar.auto_update = true;

        Ok(jar)
    }

    /// Instantiate cookie jar from a string of a Netscape formatted cookies.txt file
    pub fn from_string(contents: &String) -> Self {
        // Go through lines
        let mut cookies: HashMap<String, Cookie> = HashMap::new();
        for line in contents.split('\n') {
            if line.starts_with('#') {
                continue;
            }

            if let Some(cookie) = Cookie::from_line(line) {
                cookies.insert(cookie.name.clone(), cookie.clone());
            }
        }

        Self {
            jar_file: String::new(),
            auto_update: false,
            cookies,
        }
    }

    /// Update jar filename
    pub fn set_jar_file(&mut self, jar_file: &str) {
        self.jar_file = jar_file.to_string();
    }

    /// Change auto_update
    pub fn set_auto_update(&mut self, auto_update: bool) {
        self.auto_update = auto_update;
    }

    /// Get individual cookie
    pub fn get(&self, name: &str) -> Option<Cookie> {
        if let Some(cookie) = self.cookies.get(name) {
            return Some(cookie.clone());
        }
        None
    }

    // Set a cookie.  Will insert new cookie, or update if cookie already exists within jar.
    pub fn set(&mut self, cookie: &Cookie) {
        let name = cookie.name.clone();
        *self.cookies.entry(name.clone()).or_insert(cookie.clone()) = cookie.clone();
    }

    /// Delete a cookie within jar
    pub fn delete(&mut self, name: &str) {
        self.cookies.remove(name);
    }

    /// Clear all cookies within jar
    pub fn clear(&mut self) {
        self.cookies.clear();
    }

    /// Get http header for host
    pub fn get_http_header(&self, uri: &Url) -> Option<String> {
        // Initialize
        let mut pairs = Vec::new();
        let host = uri.host_str().unwrap();
        let host_chk = format!(".{}", host);

        // Iterate through cookies
        for (_name, cookie) in self.iter() {
            if (cookie.host != host && cookie.host != host_chk)
                || (!uri.path().starts_with(&cookie.path))
                || (cookie.secure && uri.scheme() != "https")
            {
                continue;
            }

            // Add to pairs
            let line = format!("{}={}", cookie.name, cookie.value);
            pairs.push(line);
        }

        if pairs.is_empty() {
            return None;
        }
        Some(pairs.join("; ").to_string())
    }

    /// Iterate over all cookies
    pub fn iter(&self) -> Box<dyn Iterator<Item = (String, Cookie)>> {
        Box::new(self.cookies.clone().into_iter())
    }

    /// Update cookie jar from response http headers
    pub fn update_jar(&mut self, headers: &HttpHeaders) {
        // GO through headers
        for line in headers.get_lower_vec("set-cookie") {
            // Get name and value
            let eq_index = line.find('=').unwrap_or(0);
            let sc_index = line.find(';').unwrap_or(0);
            if eq_index == 0 || sc_index == 0 || eq_index >= sc_index {
                continue;
            }
            let name = line[..eq_index].to_string();
            let value = line[eq_index + 1..sc_index].trim().to_string();

            if value.is_empty() {
                self.delete(name.as_str());
                continue;
            }

            let elem: HashMap<String, String> = line[sc_index + 1..]
                .split(';')
                .map(|e| {
                    let (mut ekey, mut evalue) = (e.to_string(), "".to_string());
                    if let Some(eindex) = e.find('=') {
                        ekey = e[..eindex].to_lowercase().trim().to_string();
                        evalue = e[eindex + 1..].trim().to_string();
                    }
                    (ekey, evalue)
                })
                .collect();

            let expires: u64 = 0;
            if let Some(_max_age) = elem.get(&"max-age".to_string()) {
                let _secs = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                //expires = secs as u64 + max_age.parse::<u64>().unwrap();
            }

            let cookie = Cookie {
                host: elem
                    .get(&"domain".to_string())
                    .unwrap_or(&"".to_string())
                    .clone(),
                path: elem
                    .get(&"path".to_string())
                    .unwrap_or(&"/".to_string())
                    .clone(),
                http_only: elem.contains_key(&"httponly".to_string()),
                secure: elem.contains_key(&"secure".to_string()),
                expires,
                name: name.to_string(),
                value: line[eq_index + 1..sc_index].trim().to_string(),
            };
            self.set(&cookie);
        }

        // Save jar file
        if self.auto_update {
            self.save_jar();
        }
    }

    /// Save jar file
    pub fn save_jar(&mut self) -> Result<(), Error> {
        if self.jar_file.is_empty() {
            return Ok(());
        }

        let mut file = match File::create(&self.jar_file) {
            Ok(r) => r,
            Err(e) => {
                return Err(Error::FileNotCreated(FileNotCreatedError {
                    filename: self.jar_file.clone(),
                    error: e.to_string(),
                }));
            }
        };
        writeln!(
            file,
            "# Auto-generated by atlas-http (https://crates.io/crates/atlas-http)\n"
        );

        // Go through all cookies
        for (_name, cookie) in self.iter() {
            writeln!(file, "{}", Cookie::to_line(&cookie));
        }

        Ok(())
    }
}
