
use openssl::error::ErrorStack;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    AES(String),
    Argon2(String),
    OpenSSL(String),
    IO(String),
    Faiss(String),
    Cfx(String),
    ProcMgr(String),
    Vault(String),
    Apollo(String),
    Onnx(String),
    Http(String),
    Json(String),
    Generic(String),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::AES(err) => write!(f, "AES-GCM error: {}", err),
            Error::OpenSSL(err) => write!(f, "OpenSSL error: {}", err),
            Error::Argon2(err) => write!(f, "Argon2 error: {}", err),
            Error::IO(err) => write!(f, "IO error: {}", err),
            Error::Faiss(err) => write!(f, "Faiss error: {}", err),
            Error::Cfx(err) => write!(f, "CFX error: {}", err),
            Error::ProcMgr(err) => write!(f, "ProcessManager error: {}", err),
            Error::Vault(err) => write!(f, "Vault error: {}", err),
            Error::Onnx(err) => write!(f, "Onnx error: {}", err),
            Error::Http(err) => write!(f, "Http error: {}", err),
            Error::Json(err) => write!(f, "Json error: {}", err),
            Error::Apollo(err) => write!(f, "Apollo-GCM error: {}", err),
            Error::Generic(msg) => write!(f, "{}", msg),
        }
    }
}

// Add support for OpenSSL ErrorStack
impl From<ErrorStack> for Error {
    fn from(err: ErrorStack) -> Self {
        Error::OpenSSL(err.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IO(e.to_string())
    }
}




