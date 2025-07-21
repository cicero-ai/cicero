#![allow(warnings)]
pub mod body;
pub mod client;
pub mod client_builder;
pub mod client_sync;
pub mod cookie;
pub mod cookie_jar;
pub mod error;
pub mod headers;
pub mod request;
pub mod response;
mod socks5;
mod tls_noverify;
mod user_agent;

use std::collections::HashMap;
use std::sync::Arc;
pub use self::client::HttpClient;
pub use self::cookie::Cookie;
pub use self::client_sync::HttpSyncClient;
pub use self::client_builder::{HttpClientConfig, HttpClientBuilder};
pub use self::request::HttpRequest;
pub use self::response::HttpResponse;
pub use self::body::HttpBody;
pub use self::headers::HttpHeaders;
pub use self::cookie_jar::CookieJar;


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProxyType {
    None,
    HTTP,
    SOCKS5,
}

