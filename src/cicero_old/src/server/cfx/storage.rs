
use omnidata::kvstore::{RocksDB, KvStore};
use crate::server::security::{Postman, UserKey};
use x25519_dalek::PublicKey;

pub struct CfxStorage {
    rocksdb: RocksDB,
    postman: Postman
}

impl CfxStorage {

    pub fn new(
        datadir: &str,
        user_key: &UserKey,
        public_key: &PublicKey
    ) -> Self {
        Self {
            rocksdb: RocksDB::new(&datadir),
            postman: user_key.to_postman(&public_key)
        }
    }

    /// Put data
    pub fn put(&self, key: &str, value: &[u8]) {
        self.rocksdb.put(key, &self.postman.encrypt(&value));
    }

    /// Retrieve data from storage
    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        
        let value: Vec<u8> = match self.rocksdb.get(key) {
            Some(r) => r,
            None => return None
        };

        Some(self.postman.decrypt(&value).unwrap())
    }

    /// Delete key from storage
    pub fn delete(&self, key: &str) -> bool {
        self.rocksdb.delete(key)
    }
}

