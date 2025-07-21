
use serde_derive::{Serialize, Deserialize};
use crate::db::{Database, DatabaseConnectionInfo, QueryDetails};
use cicero_sdk::task::iterate::IterateTaskKit;

pub struct IterateDatabaseTask {
    source: DatabaseConnectionInfo,
    query: QueryDetails
}

impl iterateDatabaseTask {
    pub fn new(source: &DatabaseConnectionInfo) -> Self {
        Self {
            source,
            query: QueryDetails::default()
        }
    }
}

impl IterateTaskKit for IterateDatabaseTask {


impl CiceroActionDetails for IterateDatabaseActionDetails {
    fn is_missing_info(&self) -> bool {
        true
    }
}



