


use std::fmt;

#[derive(Debug)]
pub enum Error {
    AES(String),
    Argon2(String),
    Generic(String),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::AES(err) => write!(f, "AES-GCM error: {}", err),
            Error::Argon2(err) => write!(f, "Argon2 error: {}", err),
            Error::Generic(msg) => write!(f, "{}", msg),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Generic(err.to_string())
    }
}



