#![allow(clippy::large_enum_variant)]

use super::{HttpHeaders, HttpRequest};
use crate::error::{Error, InvalidFirstLineError, InvalidResponseError};
use std::io::BufRead;

#[derive(Clone, Debug)]
pub struct HttpResponse {
    version: String,
    status_code: u16,
    reason: String,
    headers: HttpHeaders,
    body: String,
}

impl HttpResponse {
    /// Instantiate response with minimal properties
    pub fn new(status: &u16, headers: &Vec<String>, body: &String) -> Self {
        Self::new_full(
            status,
            &HttpHeaders::from_vec(headers),
            body,
            &"1.1".to_string(),
            &"".to_string(),
        )
    }

    /// Instantiate new response with all properties
    pub fn new_full(
        status: &u16,
        headers: &HttpHeaders,
        body: &String,
        version: &String,
        reason: &String,
    ) -> Self {
        Self {
            version: version.clone(),
            status_code: *status,
            reason: reason.clone(),
            headers: headers.clone(),
            body: body.trim().trim_end_matches('0').to_string(),
        }
    }

    /// Get protocol version
    pub fn version(&self) -> String {
        self.version.clone()
    }

    /// Get HTTP status code
    pub fn status_code(&self) -> u16 {
        self.status_code
    }

    /// Get status reason message
    pub fn reason(&self) -> String {
        self.reason.clone()
    }

    /// Get http headers
    pub fn headers(&self) -> HttpHeaders {
        self.headers.clone()
    }

    /// Get body of response
    pub fn body(&self) -> String {
        self.body.clone()
    }

    /// Get the raw response including headers and body
    pub fn raw(&self) -> String {
        let headers_str = self
            .headers
            .all()
            .iter()
            .map(|(key, value)| format!("{}: {}", key, value.join("; ")))
            .collect::<Vec<String>>()
            .join("\r\n");

        let res = format!(
            "HTTP/{} {} {}\r\n{}\n\n{}\n\n",
            self.version, self.status_code, self.reason, &headers_str, self.body
        );
        res.to_string()
    }

    /// Read first line and header of response
    pub fn read_header(
        reader: &mut Box<dyn BufRead>,
        req: &HttpRequest,
        dest_file: &str,
    ) -> Result<Self, Error> {
        // Get first line
        let mut first_line = String::new();
        match reader.read_line(&mut first_line) {
            Ok(_) => {}
            Err(e) => {
                return Err(Error::NoRead(InvalidResponseError {
                    url: req.url.clone(),
                    response: e.to_string(),
                }));
            }
        };

        // Parse first line
        let (version, status, reason) = Self::parse_first_line(&first_line, req)?;

        // Get headers
        let mut header_lines = Vec::new();
        loop {
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(_) => {}
                Err(e) => {
                    return Err(Error::NoRead(InvalidResponseError {
                        url: req.url.clone(),
                        response: e.to_string(),
                    }));
                }
            };

            if line.trim().is_empty() {
                break;
            }
            header_lines.push(line.trim().to_string());
        }
        let headers = HttpHeaders::from_vec(&header_lines);

        // Chunked transfer encoding
        if headers.has_lower("transfer-encoding")
            && headers.get_lower("transfer-encoding").unwrap().as_str() == "chunked"
        {
            let mut _tmp = String::new();
            reader.read_line(&mut _tmp).unwrap();
        }

        // Get body
        let mut body = String::new();
        if dest_file.is_empty() {
            reader.read_to_string(&mut body);
        }

        // Get response
        let res = Self::new_full(&status, &headers, &body, &version, &reason);
        Ok(res)
    }

    /// Parse first line
    pub fn parse_first_line(
        first_line: &str,
        req: &HttpRequest,
    ) -> Result<(String, u16, String), Error> {
        // Parse first line
        let mut is_valid = true;
        let parts = first_line
            .trim_start_matches("HTTP/")
            .split(' ')
            .collect::<Vec<&str>>();
        if !["1.0", "1.1", "2", "3"].contains(&parts[0]) {
            is_valid = false;
        } else if parts[1].len() != 3 || !parts[1].chars().all(|c| c.is_ascii_digit()) {
            is_valid = false;
        }

        if !is_valid {
            let error = InvalidFirstLineError {
                request: req.clone(),
                first_line: first_line.to_string(),
            };
            return Err(Error::InvalidFirstLine(error));
        }

        Ok((
            parts[0].to_string(),
            parts[1].parse::<u16>().unwrap(),
            parts[2].to_string(),
        ))
    }
}
