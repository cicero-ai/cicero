
#[cfg(feature="rocks")]
pub use self::rocksdb::RocksDB;

#[cfg(feature="sql")]
pub use self::sqldb::OpusSqlDb;

#[cfg(feature="rocks")]
mod rocksdb;

#[cfg(feature="sql")]
mod sqldb;


