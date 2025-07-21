
pub use self::rocksdb::RocksDB;

pub mod rocksdb;

pub trait KvStore {
    fn new(datadir: &str) -> Self;
    fn put(&self, key: &str, value: &[u8]);
    fn get(&self, key: &str) -> Option<Vec<u8>>;
    fn delete(&self, key: &str) -> bool;
}


