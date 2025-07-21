#![allow(warnings)]
use serde_derive::{Serialize, Deserialize};
use crate::db::DatabaseConnectionInfo;

pub mod db;
pub mod error;
pub mod kvstore;


pub enum DataSource {
    Database(DatabaseConnectionInfo),
    Directory(String)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SupportedDatabases {
    Sqlite,
    JSON,
    CSV
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SupportedKvStores {
    RocksDB
}

