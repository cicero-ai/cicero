
use cicero_sdk::api::CiceroSTD;
use crate::server::security::forge;

#[derive(Debug, Clone)]
pub struct CiceroAPI_STD {

}

impl CiceroAPI_STD {

    pub fn new() -> Self {
        Self { }
    }

}


impl CiceroSTD for CiceroAPI_STD {

    /// Encrypt
    fn encrypt(&self, message: &[u8], password: &[u8; 32]) -> Vec<u8> {
        forge::encrypt(&message, password)
    }

    /// Decrypt
    fn decrypt(&self, payload: &[u8], password: [u8; 32]) -> Option<Vec<u8>> {
        match forge::decrypt(&payload, password) {
            Ok(r) => Some(r),
            Err(_) => None
        }
    }

}


