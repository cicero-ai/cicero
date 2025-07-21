
use rocksdb::{DBCompactionStyle, DBCompressionType, DB, Options, IteratorMode, ReadOptions};
use std::fs;
use std::path::Path;
use std::sync::Arc;
use crate::Error;  // Assuming your cleaned-up Error from cicero

#[derive(Clone)]
pub struct RocksDB {
    pub db: Arc<DB>,
}

impl RocksDB {
    pub fn new(datadir: &str) -> Result<Self, Error> {
        // Ensure directory exists
        fs::create_dir_all(datadir)
            .map_err(|e| Error::Generic(format!("Failed to create {}: {}", datadir, e)))?;

        // Configure RocksDB options
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.set_compression_type(DBCompressionType::Snappy);
        // opts.increase_parallelism(num_threads) — uncomment when config’s ready

        // Open database
        let db = DB::open(&opts, datadir)
            .map_err(|e| Error::Generic(format!("Failed to open RocksDB at {}: {}", datadir, e)))?;

        Ok(Self { db: Arc::new(db) })
    }

    pub fn put(&self, key: &str, value: &[u8]) -> Result<(), Error> {
        self.db.put(key.as_bytes(), value)
            .map_err(|e| Error::Generic(format!("Put failed: {}", e)))
    }

    pub fn get(&self, key: &str) -> Result<Option<Vec<u8>>, Error> {
        self.db.get(key.as_bytes())
            .map_err(|e| Error::Generic(format!("Get failed: {}", e)))
    }

    pub fn delete(&self, key: &str) -> Result<(), Error> {
        self.db.delete(key.as_bytes())
            .map_err(|e| Error::Generic(format!("Delete failed: {}", e)))
    }

    /// Returns an iterator over all key-value pairs in the database.
    pub fn iter(&self) -> impl Iterator<Item = Result<(Box<[u8]>, Box<[u8]>), Error>> + '_ {
        let iter = self.db.iterator(IteratorMode::Start);
        iter.map(|result| {
            result.map_err(|e| Error::Generic(format!("Iterator failed: {}", e)))
        })
    }

    // Explicitly tie the iterator's lifetime to &self
    pub fn keys(&self) -> impl Iterator<Item = Result<Box<[u8]>, Error>> + '_ {
        self.iter().map(|result| result.map(|(key, _value)| key))
    }

    // Explicitly tie the iterator's lifetime to &self
    pub fn keys_with_prefix(
        &self,
        prefix: &str,
    ) -> impl Iterator<Item = Result<Box<[u8]>, Error>> + '_ {
        let mut read_opts = ReadOptions::default();
        read_opts.set_prefix_same_as_start(true);

        let iter = self.db.iterator_opt(
            IteratorMode::From(prefix.as_bytes(), rocksdb::Direction::Forward),
            read_opts,
        );

        iter.map(|result| {
            result.map(|(key, _value)| key)
                .map_err(|e| Error::Generic(format!("Prefix iterator failed: {}", e)))
        })
    }
}

