use std::fmt;

#[derive(Debug)]
pub enum Error {
    Save(String),
    Load(String),
    Generic(String),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Save(err) => write!(f, "Save error: {}", err),
            Error::Load(err) => write!(f, "Load error: {}", err),
            Error::Generic(msg) => write!(f, "{}", msg),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Generic(err.to_string())
    }
}
