
use serde::{Serialize, Deserialize};
use std::fs;
use std::env;
use std::path::PathBuf;
use bincode;
use hex;
use super::{forge, CiceroHQ};

#[derive(Serialize, Deserialize, Clone)]
pub struct License {
    pub license_id: String,
    pub product: String,
    pub client_name: String,
    pub encryption_key: [u8; 32]
}

#[derive(Serialize, Deserialize)]
pub struct ValidationApiResponse {
    pub is_valid: bool,
    pub encryption_key: String
}

impl License {

    pub fn new(license_id: &str, product: &str, client_name: &str) -> Self {
        Self {
            license_id: license_id.to_string(),
            product: product.to_string(),
            client_name: client_name.to_string(),
            encryption_key: [0; 32]
        }
    }

    /// Load license
    pub fn load() -> Self {
        let filename = "/home/boxer/devel/cicero/cicero/src/verax/data/local/license.dat";
        //let bytes = include_bytes!("/home/boxer/devel/cicero/cicero/src/verax/data/local/license.dat");
        let bytes = fs::read(&filename).unwrap();
        let license: License = bincode::deserialize(&bytes[..]).unwrap();
        license
    }

    #[cfg(feature="local")]
    /// Load local license
    pub fn load_local() -> Self {
        let bytes = include_bytes!("/home/boxer/devel/cicero/cicero/src/verax/data/local/license.dat");
        let license: License = bincode::deserialize(&bytes[..]).unwrap();
        license
    }

    // Save
    pub fn save(&self, filename: &str) {
        let mut tmp = self.clone();
        tmp.encryption_key = [0; 32];
        fs::write(&filename, bincode::serialize(&tmp).unwrap()).unwrap();
    }

    #[cfg(feature="local")]
    /// Save local
    pub fn save_local(&self, filename: &str) {
        fs::write(&filename, bincode::serialize(&self).unwrap()).unwrap();
    }

    /// Convert into api

    /// Validate 
    pub fn validate(&mut self) -> bool {

        // Send request
        let req_data = format!("license_id={}", self.license_id);
        let hq = CiceroHQ::new();
        let res = hq.send::<ValidationApiResponse>("license/validate", &req_data);

        // Check for invalid
        if !res.is_valid {
            println!("License is invalid, aborting.");
            return false;
        }

        // Get encryption key
        let key_bytes = hex::decode(&res.encryption_key).unwrap();
        //self.encryption_key = key_bytes.try_into().unwrap();

        true
    }

    /// Encrypt
    fn encrypt(&self, message: &[u8]) -> Vec<u8> {
        forge::encrypt(&message, &self.encryption_key.clone())
    }

    /// Decrypt
    fn decrypt(&self, payload: &[u8]) -> Option<Vec<u8>> {
        match forge::decrypt(&payload, self.encryption_key.clone()) {
            Ok(r) => Some(r),
            Err(_) => None
        }
    }

}


