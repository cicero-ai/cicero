

use std::fmt;

#[derive(Debug)]
pub enum Error {
    NoConfig(String),
    ApiInvalidResponse(String),
    ApiNoResults(bool),
    RpcServer(String),
    RpcClient(String),
    Sql((String, String)),
    InvalidUrl(String),
    LLM((String, String)),
    InvalidPassword,
    ApolloConnectTimeout,
    Generic(String)
}


impl std::error::Error for Error { }
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match self {
            Error::NoConfig(var_name) => write!(f, "Confirmation variable does not exist, {}", var_name),
            Error::ApiInvalidResponse(err) => write!(f, "Did not receive valid response from API, error: {}", err),
            Error::ApiNoResults(err) => write!(f, "API did not return any choices / results."), 
            Error::RpcServer(err) => write!(f, "RPC Server: {}", err),
            Error::RpcClient(err) => write!(f, "RPC Client: {}", err),
            Error::Sql((sql, err)) => write!(f, "Sql Error:  Unable to execute SQL statement:\n\n    {}\n\n    Error: {}\n\n", sql, err),
            Error::LLM((task, err)) => write!(f, "LLM Error while performing '{}', error: {}", task, err),
            Error::InvalidPassword => write!(f, "Invalid encryption password, please try again."),
            Error::ApolloConnectTimeout => write!(f, "Unable to connect to Apollo server, it appears to be down."),
            Error::Generic(err) => write!(f, "{}", err),
            _ => write!(f, "Unknown generic error")
        }

    }
}





