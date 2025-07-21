use super::{HttpBody, HttpClientConfig, HttpHeaders, ProxyType};
use crate::error::Error;
use url::Url;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;
//use std::io::BufReader as TokioBufReader;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncBufRead;

#[derive(Clone, Debug)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: HttpHeaders,
    pub body: HttpBody,
}

impl HttpRequest {
    pub fn new(method: &str, url: &str, headers: &Vec<&str>, body: &HttpBody) -> Self {
        Self {
            method: method.to_uppercase().to_string(),
            url: url.to_string(),
            headers: HttpHeaders::from_vec(&headers.iter().map(|s| s.to_string()).collect()),
            body: body.clone(),
        }
    }

    // Validate URL and scheme
    pub fn prepare(&self, config: &HttpClientConfig) -> Result<(Url, u16, Vec<u8>), Error> {
        // Parse url
        let uri = match Url::parse(&self.url) {
            Ok(r) => r,
            Err(_err) => {
                return Err(Error::InvalidUri(self.url.clone()));
            }
        };

        // Check scheme
        if uri.scheme() != "http" && uri.scheme() != "https" {
            return Err(Error::ProtoNotSupported(uri.scheme().to_string()));
        }

        // Get port
        let mut _port: u16 = 0;
        if uri.port().is_none() && uri.scheme() == "https" {
            _port = 443;
        } else if uri.port().is_none() && uri.scheme() == "http" {
            _port = 80;
        } else {
            _port = uri.port().unwrap();
        }

        // Generate message
        let message = self.generate_raw(config, &uri);

        Ok((uri, _port, message))
    }

    /// Generate raw HTTP message to be sent
    fn generate_raw(&self, config: &HttpClientConfig, uri: &Url) -> Vec<u8> {
        // Get target
        let mut target = uri.path().to_string();
        if let Some(query) = uri.query() {
            target = format!("{}?{}", target, query);
        }

        // Modify target for proxy, if needed
        if config.proxy_type != ProxyType::None {
            target = format!(
                "{}://{}{}",
                uri.scheme(),
                uri.host_str().unwrap(),
                uri.path()
            );
        }

        let mut lines = vec![
            format!("{} {} HTTP/1.1", &self.method, target),
            format!("Host: {}", uri.host_str().unwrap()),
        ];

        if let Some(ua) = &config.user_agent {
            lines.push(format!("User-Agent: {}", ua));
        }

        // HTTP client headers
        for (key, value) in config.headers.all().iter() {
            lines.push(format!("{}: {}", key, value.join("; ")));
        }

        // Cookie header
        if let Some(cookie_hdr) = config.cookie.get_http_header(uri) {
            lines.push(format!("Cookie: {}", cookie_hdr));
        }

        // POST headers
        if !self.body.files().is_empty() && !self.headers.has_lower("content-type") {
            lines.push(format!(
                "Content-type: multipart/form-data; boundary={}",
                self.body.boundary()
            ));
        } else if self.body.is_form_post() && !self.headers.has_lower("content-type") {
            lines.push("Content-type: application/x-www-form-urlencoded".to_string());
        }

        // Format post body, if needed
        let mut post_body: Vec<u8> = Vec::new();
        if self.body.is_form_post() {
            post_body = self.body.format();
            lines.push(format!("Content-length: {}", post_body.len()));
        }

        // HTTP request headers
        for (key, value) in self.headers.all().iter() {
            lines.push(format!("{}: {}", key, value.join("; ")));
        }
        lines.push("\r\n".to_string());

        // Add body
        let mut message = lines.join("\r\n").as_bytes().to_vec();
        message.extend(post_body);
        message.extend_from_slice("\r\n".as_bytes());

        message
    }

    /// Build from buf reader
    pub fn build(stream: &mut TcpStream) -> Result<Self, Error> {

        // Get first line
        let mut reader = BufReader::new(stream);
        let mut first_line = String::new();
        match reader.read_line(&mut first_line) {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom("Invalid first line".to_string()))
        };

        // Parse first line
        let (method, path) = Self::parse_first_line(&first_line)?;

        // Get headers
        let mut header_lines = Vec::new();
        loop {
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(_) => {}
                Err(e) => return Err(Error::Custom("Unable to read from incoming connection.".to_string()))
            };

            if line.trim().is_empty() {
                break;
            }
            header_lines.push(line.trim().to_string());
        }
        let headers = HttpHeaders::from_vec(&header_lines);

        // Read body from buffer
        let length: usize = headers.get_lower_line("content-length").unwrap_or("0".to_string()).parse::<usize>().unwrap();
        let mut body_bytes = vec![0; length];
        let bytes_read = reader.read(&mut body_bytes).unwrap();
        let body_str: String = String::from_utf8_lossy(&body_bytes).to_string();

        // Get body
        let body = if headers.has_lower("content-type") && headers.get_lower_line("content-type").unwrap() == "application/x-www-form-urlencoded".to_string() {
            HttpBody::from_string(&body_str.as_str())
        } else {
            HttpBody::from_raw(&body_bytes)
        };

        // Return
        Ok( Self {
            method,
            url: format!("http://127.0.0.1{}", path),
            headers,
            body
        })

    }

    /// Build request from stream asynchronously
    pub async fn build_async(stream: &mut tokio::net::TcpStream) -> Result<Self, Error> {

        // Read into buffer
        //let (reader, mut writer) = tokio::io::split(stream);
        let mut reader = tokio::io::BufReader::new(stream);

        // Get first line
        let mut first_line = String::new();
        match reader.read_line(&mut first_line).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom("Invalid first line".to_string()))
        };

        // Parse first line
        let (method, path) = Self::parse_first_line(&first_line)?;

        // Get headers
        let mut header_lines = Vec::new();
        loop {
            let mut line = String::new();
            let n = match reader.read_line(&mut line).await {
                Ok(r) => r,
                Err(e) => return Err(Error::Custom("Unable to read from incoming connection.".to_string()))
            };

            if n == 0 || line.trim().is_empty() {
                break;
            }
            header_lines.push(line.trim().to_string());
        }
        let headers = HttpHeaders::from_vec(&header_lines);

        // Read body from buffer
        let length: usize = headers.get_lower_line("content-length").unwrap_or("0".to_string()).parse::<usize>().unwrap();
        let mut body_bytes = vec![0; length];
        let mut body_str = String::new();

        if length > 0 {
            let body_bytes = reader.fill_buf().await.unwrap();
            body_str = String::from_utf8_lossy(&body_bytes).to_string();
        }

        // Get body
        let body = if headers.has_lower("content-type") && headers.get_lower_line("content-type").unwrap() == "application/x-www-form-urlencoded".to_string() {
            HttpBody::from_string(&body_str.as_str())
        } else {
            HttpBody::from_raw(&body_bytes)
        };

        // Return
        Ok( Self {
            method,
            url: format!("http://127.0.0.1{}", path),
            headers,
            body
        })

    }

    /// Build request from stream asynchronously
    pub async fn build_async_tls(stream: &mut tokio_rustls::server::TlsStream<tokio::net::TcpStream>) -> Result<Self, Error> {

        // Read into buffer
        //let (reader, mut writer) = tokio::io::split(stream);
        let mut reader = tokio::io::BufReader::new(stream);

        // Get first line
        let mut first_line = String::new();
        match reader.read_line(&mut first_line).await {
            Ok(_) => {}
            Err(e) => return Err(Error::Custom("Invalid first line".to_string()))
        };

        // Parse first line
        let (method, path) = Self::parse_first_line(&first_line)?;

        // Get headers
        let mut header_lines = Vec::new();
        loop {
            let mut line = String::new();
            let n = match reader.read_line(&mut line).await {
                Ok(r) => r,
                Err(e) => return Err(Error::Custom("Unable to read from incoming connection.".to_string()))
            };

            if n == 0 || line.trim().is_empty() {
                break;
            }
            header_lines.push(line.trim().to_string());
        }
        let headers = HttpHeaders::from_vec(&header_lines);

        // Read body from buffer
        let length: usize = headers.get_lower_line("content-length").unwrap_or("0".to_string()).parse::<usize>().unwrap();
        let mut body_bytes = vec![0; length];
        let mut body_str = String::new();

        if length > 0 {
            let body_bytes = reader.fill_buf().await.unwrap();
            body_str = String::from_utf8_lossy(&body_bytes).to_string();
        }

        // Get body
        let body = if headers.has_lower("content-type") && headers.get_lower_line("content-type").unwrap() == "application/x-www-form-urlencoded".to_string() {
            HttpBody::from_string(&body_str.as_str())
        } else {
            HttpBody::from_raw(&body_bytes)
        };

        // Return
        Ok( Self {
            method,
            url: format!("http://127.0.0.1{}", path),
            headers,
            body
        })

    }

    /// Parse first line
    pub fn parse_first_line(first_line: &str) -> Result<(String, String), Error> {

        // Split into parts
        let parts = first_line.split(" ").collect::<Vec<&str>>();
        if parts.len() != 3 {
            return Err(Error::Custom("Invalid first line.".to_string()));
        } else if !parts[2].starts_with("HTTP/") {
            return Err(Error::Custom("Invalid first line.".to_string()));
        } else if !vec!["GET","POST","PUT","DELETE","HEAD","OPTIONS"].contains(&parts[0].to_uppercase().as_str()) {
            return Err(Error::Custom("Invalid first line.".to_string()));
        }

        // Validate path
        let url = match Url::parse(&format!("http://example.com{}", parts[1])) {
            Ok(url) => url,
            Err(_) => return Err(Error::Custom("Invalid first line.".to_string()))
        };

        // Return
        Ok((parts[0].to_uppercase().to_string(), parts[1].to_string()))
    }

}


