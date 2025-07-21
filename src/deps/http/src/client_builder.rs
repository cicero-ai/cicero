use base64::{engine::general_purpose::STANDARD, Engine as _};
use rustls::{ClientConfig, RootCertStore};
use std::path::Path;
use std::sync::Arc;
use super::{CookieJar, HttpClient, HttpHeaders, HttpSyncClient, ProxyType};
use crate::{tls_noverify, user_agent};

#[derive(Debug, Clone)]
pub struct HttpClientConfig {
    pub tls_config: Arc<rustls::ClientConfig>,
    pub user_agent: Option<String>,
    pub headers: HttpHeaders,
    pub cookie: CookieJar,
    pub follow_location: bool,
    pub timeout: u64,
    pub proxy_type: ProxyType,
    pub proxy_host: String,
    pub proxy_port: u16,
    pub proxy_user: String,
    pub proxy_password: String,
}

pub struct HttpClientBuilder {
    config: HttpClientConfig,
}

impl Default for HttpClientBuilder {
    fn default() -> HttpClientBuilder {
        Self::new()
    }
}

impl HttpClientBuilder {
    pub fn new() -> Self {
        Self {
            config: HttpClientConfig::default()
        }
    }

    /// Finish building, and return asynchronous HTTP client
    pub fn build_async(&mut self) -> HttpClient {
        HttpClient::new(&self.config)
    }

    /// Finish building, and return blocking synchronous HTTP client
    pub fn build_sync(&mut self) -> HttpSyncClient {
        HttpSyncClient::new(&self.config)
    }

    /// Will always follow Location headers it encounters
    pub fn follow_location(mut self) -> Self {
        self.config.follow_location = true;
        self
    }

    // Set timeout limit in seconds
    pub fn timeout(mut self, seconds: u64) -> Self {
        self.config.timeout = seconds;
        self
    }

    /// Cookie jar file, will be auto-maintained unless you change auto-update to false via CookieJar::set_auto_update(bool) method.
    pub fn cookie_jar(mut self, jar_file: &str) -> Self {
        if !Path::new(&jar_file).exists() {
            self.config.cookie.set_jar_file(jar_file);
            self.config.cookie.set_auto_update(true);
        } else {
            self.config.cookie = CookieJar::from_file(jar_file, true).unwrap();
        }
        self
    }

    /// Set cookies from contents / lines of a Netscape formatted cookies.txt file
    pub fn cookie_string(mut self, cookie_str: &str) -> Self {
        self.config.cookie = CookieJar::from_string(&cookie_str.to_string());
        self
    }

    /// Do not verify SSL certificates
    pub fn noverify_ssl(mut self) -> Self {
        // Initialize root store
        let mut root_store = RootCertStore::empty();
        root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

        // Create config
        let mut tls_config = ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        tls_config.dangerous().set_certificate_verifier(Arc::new(
            tls_noverify::NoCertificateVerification::new(rustls::crypto::ring::default_provider()),
        ));

        self.config.tls_config = Arc::new(tls_config);
        self
    }

    /// Define user agent for session
    pub fn user_agent(mut self, user_agent: &str) -> Self {
        self.config.user_agent = Some(user_agent.to_string());
        self
    }

    /// Set base headers to more closely emulate a web browser.
    pub fn browser(mut self) -> Self {
        // Create headers
        self.config.headers = HttpHeaders::new();
        self.config.headers.set(
            "Accept",
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8",
        );
        self.config.headers.set("Accept-Language", "en-US,en;q=0.5");
        self.config.headers.set("Accept-Encoding", "identity");
        self.config.headers.set("Connection", "close");

        // User agent
        if self.config.user_agent.is_none() {
            self.config.user_agent = Some(user_agent::random());
        }
        self.config.follow_location = true;
        self
    }

    // Define basic HTTP authentication
    pub fn basic_auth(mut self, user: &str, password: &str) -> Self {
        // Disable authentication, fi needed
        if user.is_empty() {
            self.config.headers.delete("Authorization");
            return self;
        }

        // Enable authentication
        let auth_userpass = format!("{}:{}", user, password);
        let auth_line = format!("Basic {}", STANDARD.encode(auth_userpass));
        self.config.headers.set("Authorization", auth_line.as_str());

        self
    }

    /// Send requests over the Tor network.
    pub fn tor(mut self) -> Self {
        self.config.proxy_host = "127.0.0.1".to_string();
        self.config.proxy_port = 9050;
        self.config.proxy_type = ProxyType::SOCKS5;
        self
    }

    // Send requests through a HTTP / SOCKS5 proxy
    pub fn proxy(mut self, host: &str, port: &u16) -> Self {
        if self.config.proxy_type == ProxyType::None {
            self.config.proxy_type = ProxyType::SOCKS5;
        }
        self.config.proxy_host = host.to_string();
        self.config.proxy_port = *port;
        self
    }

    // Define authentication for the HTTP / SOCKS5 proxy
    pub fn proxy_auth(mut self, user: &str, password: &str) -> Self {
        self.config.proxy_user = user.to_string();
        self.config.proxy_password = password.to_string();

        if self.config.proxy_user.is_empty() {
            self.config.headers.delete("Proxy-Authorization");
        } else {
            let auth_userpass = format!("{}:{}", user, password);
            let auth_line = format!("Basic {}", STANDARD.encode(auth_userpass));
            self.config
                .headers
                .set("Proxy-Authorization", auth_line.as_str());
        }

        self
    }

    /// Define whether it's a HTTP or SOCKS5 proxy
    pub fn proxy_type(mut self, proxy_type: ProxyType) -> Self {
        self.config.proxy_type = proxy_type;
        self
    }
}

impl Default for HttpClientConfig {
    fn default() -> HttpClientConfig {

        // Initialize root store
        let mut root_store = RootCertStore::empty();
        root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

        // Create config
        let tls_config = ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        HttpClientConfig {
            tls_config: Arc::new(tls_config),
            user_agent: None,
            headers: HttpHeaders::from_vec(&vec!["Connection: close".to_string()]),
            cookie: CookieJar::new(),
            follow_location: false,
            timeout: 5,
            proxy_type: ProxyType::None,
            proxy_host: String::new(),
            proxy_port: 0,
            proxy_user: String::new(),
            proxy_password: String::new(),
        }

    }
}



