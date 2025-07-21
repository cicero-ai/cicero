#![allow(clippy::large_enum_variant)]

use super::{
    HttpBody, HttpClientConfig, HttpRequest, HttpResponse, HttpSyncClient, ProxyType,
};
use crate::client_builder::HttpClientBuilder;
use crate::error::{Error, FileNotCreatedError, InvalidResponseError};
use crate::socks5;
use rustls::pki_types::ServerName;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use url::Url;

#[derive(Debug, Clone)]
pub struct HttpClient {
    pub config: HttpClientConfig,
}

impl HttpClient {
    pub fn new(config: &HttpClientConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }

    /// Instantiate HTTP client builder
    pub fn builder() -> HttpClientBuilder {
        HttpClientBuilder::new()
    }

    /// Send HTTP request, and return response
    pub async fn send(&mut self, req: &HttpRequest) -> Result<HttpResponse, Error> {
        self.send_request(req, &String::new()).await
    }

    /// Download a file
    pub async fn download(&mut self, url: &str, dest_file: &str) -> Result<HttpResponse, Error> {
        let req = HttpRequest::new("GET", url, &vec![], &HttpBody::empty());
        self.send_request(&req, &dest_file.to_string()).await
    }

    /// Send GET request
    pub async fn get(&mut self, url: &str) -> Result<HttpResponse, Error> {
        let req = HttpRequest::new("GET", url, &Vec::new(), &HttpBody::empty());
        self.send_request(&req, &String::new()).await
    }

    /// Send POST request
    pub async fn post(&mut self, url: &str, body: &HttpBody) -> Result<HttpResponse, Error> {
        let req = HttpRequest::new("POST", url, &Vec::new(), body);
        self.send_request(&req, &String::new()).await
    }

    /// Send PUT request
    pub async fn put(&mut self, url: &str, data: &[u8]) -> Result<HttpResponse, Error> {
        let req = HttpRequest::new("PUT", url, &Vec::new(), &HttpBody::from_raw(data));
        self.send_request(&req, &String::new()).await
    }

    /// Send DELETE request
    pub async fn delete(&mut self, url: &str) -> Result<HttpResponse, Error> {
        let req = HttpRequest::new("DELETE", url, &Vec::new(), &HttpBody::empty());
        self.send_request(&req, &String::new()).await
    }

    /// Send OPTIONS request
    pub async fn options(&mut self, url: &str) -> Result<HttpResponse, Error> {
        let req = HttpRequest::new("OPTIONS", url, &Vec::new(), &HttpBody::empty());
        self.send_request(&req, &String::new()).await
    }

    /// Send HEAD request
    pub async fn head(&mut self, url: &str) -> Result<HttpResponse, Error> {
        let req = HttpRequest::new("HEAD", url, &Vec::new(), &HttpBody::empty());
        self.send_request(&req, &String::new()).await
    }

    // Send request, used internally by the other methods.
    async fn send_request(
        &mut self,
        req: &HttpRequest,
        dest_file: &String,
    ) -> Result<HttpResponse, Error> {
        // Prepare uri and http message
        let (uri, port, message) = req.prepare(&self.config)?;

        // Connect
        let mut reader = self.connect(&uri, &port, &message).await?;

        // Read header
        let mut res = HttpResponse::read_header(&mut reader, req, dest_file)?;
        self.config.cookie.update_jar(&res.headers());

        // Check follow location
        if self.config.follow_location && res.headers().has_lower("location") {
            res = self.follow(&res, dest_file)?;
        }

        // Return if not downloading a file
        if dest_file.is_empty() {
            return Ok(res);
        }

        // Save output file
        let dest_path = Path::new(&dest_file);
        let mut fh = match File::create(dest_path) {
            Ok(r) => r,
            Err(e) => {
                return Err(Error::FileNotCreated(FileNotCreatedError {
                    filename: dest_file.to_string(),
                    error: e.to_string(),
                }));
            }
        };

        // Save file
        let mut buffer = [0u8; 2048];
        loop {
            let bytes_read = match reader.read(&mut buffer) {
                Ok(r) => r,
                Err(e) => {
                    return Err(Error::NoRead(InvalidResponseError {
                        url: req.url.clone(),
                        response: e.to_string(),
                    }));
                }
            };

            if bytes_read == 0 {
                break;
            }
            fh.write_all(&buffer).unwrap();
        }

        Ok(res)
    }

    /// Check redirect if follow_location enabled
    fn follow(&self, res: &HttpResponse, dest_file: &String) -> Result<HttpResponse, Error> {
        let redirect_url = res.headers().get_lower("location").unwrap();
        let mut rhttp = HttpSyncClient::new(&self.config.clone());

        let next_res = if dest_file.is_empty() {
            rhttp.get(&redirect_url.clone())?
        } else {
            rhttp.download(&redirect_url.clone(), dest_file)?
        };

        Ok(next_res)
    }

    // Connect to remote server
    pub async fn connect(
        &self,
        uri: &Url,
        port: &u16,
        message: &[u8],
    ) -> Result<Box<dyn BufRead>, Error> {
        // Prepare uri
        let hostname =
            if self.config.proxy_type != ProxyType::None && !self.config.proxy_host.is_empty() {
                format!("{}:{}", self.config.proxy_host, self.config.proxy_port)
            } else {
                format!("{}:{}", &uri.host_str().unwrap(), port)
            };
        let mut address = hostname.to_socket_addrs().unwrap();
        let addr = address.next().unwrap();

        // Open tcp stream
        let mut sock =
            match TcpStream::connect_timeout(&addr, Duration::from_secs(self.config.timeout)) {
                Ok(r) => r,
                Err(_e) => {
                    return Err(Error::NoConnect(hostname.clone()));
                }
            };
        sock.set_nodelay(true).unwrap();

        // SOCKs5 connection, if needed
        if self.config.proxy_type == ProxyType::SOCKS5 {
            socks5::connect(&mut sock, &self.config, uri, port);
        }

        // Connect over SSL, if needed
        if uri.scheme() == "https" && self.config.proxy_type != ProxyType::HTTP {
            let dns_name = ServerName::try_from(uri.host_str().unwrap())
                .unwrap()
                .to_owned();
            let conn = rustls::ClientConnection::new(Arc::clone(&self.config.tls_config), dns_name)
                .unwrap();

            let mut tls_stream = rustls::StreamOwned::new(conn, sock);
            tls_stream.flush().unwrap();
            tls_stream.write_all(message).unwrap();

            let reader = BufReader::with_capacity(2048, tls_stream);
            return Ok(Box::new(reader));
        }

        // Get reader
        sock.write_all(message).unwrap();
        let reader = BufReader::with_capacity(2048, sock);

        Ok(Box::new(reader))
    }
}
