//use std::error::Error;
use super::HttpRequest;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidResponse(InvalidResponseError),
    InvalidUri(String),
    ProtoNotSupported(String),
    NoConnect(String),
    NoRead(InvalidResponseError),
    NoWrite(String),
    InvalidFirstLine(InvalidFirstLineError),
    Io(std::io::Error),
    FileNotExists(String),
    FileNotCreated(FileNotCreatedError),
    Custom(String),
}

#[derive(Debug)]
pub struct InvalidResponseError {
    pub url: String,
    pub response: String,
}

#[derive(Debug)]
pub struct InvalidFirstLineError {
    pub request: HttpRequest,
    pub first_line: String,
}
#[derive(Debug)]
pub struct FileNotCreatedError {
    pub filename: String,
    pub error: String,
}

impl std::error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidResponse(err) => write!(f, "InvalidResponse: Did not receive valid HTTP response from {}", err.url),
            Error::InvalidUri(url) => write!(f, "InvalidUri: The supplied URL is invalid, {}", url),
            Error::ProtoNotSupported(proto) => write!(f, "The '{}://' protocol is not supported.  Only the https:// and http:// protocols are supported.", proto),
            Error::NoConnect(host) => write!(f, "Unable to connect to server at {}", host),
            Error::NoRead(err) => write!(f, "Unable to read from server at URL {}, server error: {}", err.url, err.response),
            Error::NoWrite(err) => write!(f, "Unable to write to server, server error: {}", err),
            Error::InvalidFirstLine(err) => write!(f, "Received malformed first line within response: {}", err.first_line),
            Error::Io(err) => write!(f, "HTTP IO: {}", err),
            Error::FileNotExists(file_path) => write!(f, "Unable to upload file, as file does not exist at {}", file_path),
        Error::FileNotCreated(err) => write!(f, "Unable to create file at {}, error: {}", err.filename, err.error),
            Error::Custom(err) => write!(f, "HTTP Error: {}", err)
        }
    }
}
