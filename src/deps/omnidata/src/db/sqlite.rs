
use rusqlite::{Connection, Rows, params};
use rusqlite::types::ToSql;
use std::sync::Arc;
use crate::error::Error;
use super::DatabaseReader;

pub struct Sqlite {
    pub conn: Connection,
}

impl Sqlite {

    pub fn new(dbfile: &str) -> Arc<Self> {

        let conn = Connection::open(&dbfile).unwrap_or_else(|_err| {
            panic!("Unable to open SQLite database");
        });

    Arc::new(Self { conn })
    }

    /// Execute SQL query
    pub fn execute(&self, sql: &str, params: &[&str]) -> Result<(), Error> {

        match self.conn.execute(&sql, []) {
            Ok(_) => { },
            Err(e) => return Err(Error::SqlQuery((sql.to_string(), e)))
        };

        Ok(())
    }


}

impl DatabaseReader for Sqlite {

    fn query(&self, sql: &str, params: &Vec<String>) {

    }

}


