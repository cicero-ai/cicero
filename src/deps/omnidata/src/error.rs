
use std::fmt;
use rusqlite::Error as SqliteError;

#[derive(Debug)]
pub enum Error {
    NoConfig(String),
    NoDriver,
    SqlQuery((String, SqliteError)),
    Generic(String)
}


impl std::error::Error for Error { }
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match self {
            Error::NoConfig(var_name) => write!(f, "Confirmation variable does not exist, {}", var_name),
            Error::NoDriver => write!(f, "No supported database driver specified"),
            Error::SqlQuery((sql, e)) => write!(f, "Unable to execute SQL statement {}\n\n error: {}", sql, e.to_string()),
            Error::Generic(err) => write!(f, "HTTP Error: {}", err),
            _ => write!(f, "An unknown error within the omnidata crate occured.")
        }

    }
}





