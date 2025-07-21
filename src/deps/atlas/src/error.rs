
use std::fmt;
use crate::scraper::LayoutComponent;

#[derive(Debug)]
pub enum Error {
    Scraper(String),
    InvalidUri(String),
    IrrelevantPage(String),
    IndeterminateLayout(LayoutComponent),
    Custom(String)
}

#[derive(Debug)]
pub struct InvalidResponseError {
    pub request: String,
    pub response: String
}

impl std::error::Error for Error { }
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match self {
            Error::Scraper(err) => write!(f, "Scraper: {}", err),
            Error::InvalidUri(url) => write!(f, "Invalid URL specified, {}", url),
            Error::IrrelevantPage(err) => write!(f, "HTTP Error: {}", err),
            Error::IndeterminateLayout(comp) => write!(f, "Unable to determine page layout."),
            Error::Custom(err) => write!(f, "HTTP Error: {}", err)
        }

    }
}




