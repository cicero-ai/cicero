
use atlas_http::HttpRequest;
use super::{Vault, AuthUser};
use crate::Error;

#[derive(Clone)]
pub struct Authenticator { }

impl Authenticator {
    pub fn new() -> Self {
        Self { }
    }

    pub fn check_http_req(&self, http_req: &HttpRequest, valut: &Vault) -> Result<AuthUser, Error> {
        Ok(None)
    }

    pub fn check_api_key(&self, api_key: &str, valut: &Vault) -> Result<AuthUser, Error> {
        Ok(None)
    }

}



