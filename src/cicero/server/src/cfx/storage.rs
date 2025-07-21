
use opus::db::RocksDB;
use cicero::security::{Postman, UserKey};
use cicero::preludes::*;
use x25519_dalek::PublicKey;
use crate::Error;

pub struct CfxStorage {
    rocksdb: RocksDB,
    postman: Postman
}

impl CfxStorage {
    pub fn new(
        datadir: &str,
        user_key: &UserKey,
        public_key: &PublicKey
    ) -> Result<Self, Error> {
        let storage = Self {
            rocksdb: RocksDB::new(&datadir).map_err(|e| Error::Cfx(e.to_string()) )?,
            postman: user_key.to_postman(&public_key)
        };
        Ok(storage)
    }

    /// Put data
    pub fn put(&self, key: &str, value: &[u8]) -> Result<(), Error> {
        self.rocksdb.put(key, &self.postman.encrypt(&value)?.as_slice())
            .map_err(|e| Error::Cfx(e.to_string()) )
    }

    /// Retrieve data from storage
    pub fn get(&self, key: &str) -> Result<Option<Vec<u8>>, Error> {
        let value: Vec<u8> = match self.rocksdb.get(key).map_err(|e| Error::Cfx(e.to_string()) )? {
            Some(r) => r,
            None => return Ok(None)
        };

        let decrypted = self.postman.decrypt(&value)?;
            Ok(Some(decrypted))
    }

    /// Delete key from storage
    pub fn delete(&self, key: &str) -> Result<(), Error> {
        self.rocksdb.delete(key)
            .map_err(|e| Error::Cfx(e.to_string()) )
    }
}

