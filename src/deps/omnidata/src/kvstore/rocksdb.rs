
//extern crate rocksdb;

use rocksdb::{DBCompactionStyle, DBCompressionType, DB};
use std::fs;
use std::path::Path;
use std::sync::Arc;
use super::KvStore;

#[derive(Clone)]
pub struct RocksDB {
    pub db: Arc<DB>,
}

impl KvStore for RocksDB {

    fn new(datadir: &str) -> Self {

        // Create directory, if needed
        if !Path::new(&datadir).exists() {
            match fs::create_dir_all(&datadir) {
                Ok(_) => {}
                Err(_error) => panic!("Unable to create directory at {}.", datadir),
            };
        }

        // Set options
        let mut opts = rocksdb::Options::default();
        opts.create_if_missing(true);
        opts.set_compression_type(DBCompressionType::Snappy);
        //opts.increase_parallelism(&CONFIG.num_threads);

        // Connect to database
        let database = match DB::open(&opts, datadir) {
            Ok(r) => r,
            Err(e) => panic!("Unable to open RocksDB at {}, error: {}", datadir, e)
        };

        // Return
        RocksDB {
            db: Arc::new(database),
        }
    }

    fn put(&self, key: &str, value: &[u8]) {
        self.db.put(key.as_bytes(), value).unwrap();
    }

    fn get(&self, key: &str) -> Option<Vec<u8>> {
        let value: Vec<u8> = match self.db.get(key.as_bytes()) {
            Ok(Some(r)) => r.to_vec(),
            Ok(None) => return None,
            Err(e) => panic!("Received database error retrieving key, error: {}", e)
        };

        Some(value)
    }

    fn delete(&self, key: &str) -> bool {
        self.db.delete(key.as_bytes()).is_ok()
    }
}
