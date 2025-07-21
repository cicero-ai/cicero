

use omnidata::db::Sqlite;
use omnidata::error::Error as OmnidataError;
use std::sync::Arc;

pub struct UserSqlDb {
    sqldb: Arc<Sqlite>
}

impl UserSqlDb {

    pub fn new(dbfile: &str) -> Self {
        Self {
            sqldb: Sqlite::new(&dbfile)
        }
    }

    /// Execute
    pub fn execute(&self, sql: &str, params: &[&str]) -> Result<(), OmnidataError> {
        self.sqldb.execute(&sql, &params)?;
        Ok(())
    }

}


