

pub use self::sqlite::Sqlite;
use serde_derive::{Serialize, Deserialize};
use crate::error::Error;
use super::SupportedDatabases;

pub mod sqlite;

pub trait DatabaseReader {
    fn query(&self, sql: &str, params: &Vec<String>);
}
pub struct Database { }

impl Database {

    //pub fn connect(info: &DatabaseConnectionInfo) -> Result<Box<dyn DatabaseReader>, Error> {

        // Check for no driver
        //if info.driver.is_none() {
            //return Err(Error::NoDriver);
        //}
        //let driver = info.driver.unwrap();

        // Try to connect
        //let res: Box<dyn DatabaseReader> = match driver {
            //SupportedDatabases::Sqlite => Sqlite::new(&info.dbname.as_str()),
            //_ => Err(Error::NoDriver)
        //};

        //res
    //}

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DatabaseDriver {
    mySQL,
    PostgreSQL,
    SQLite,
    JSON,
    CSV,
    TabDelimited
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConnectionInfo {
    driver: Option<DatabaseDriver>,
    name: String,
    user: String,
    password: String,
    host: String,
    port: u16
}

impl Default for DatabaseConnectionInfo {

    fn default() -> DatabaseConnectionInfo {
        DatabaseConnectionInfo {
            driver: None,
            name: String::new(),
            user: String::new(),
            password: String::new(),
            host: String::new(),
            port: 0
        }
    }

}


