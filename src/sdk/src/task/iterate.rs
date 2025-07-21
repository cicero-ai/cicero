
use serde_derive::{Serialize, Deserialize};
use omnidata::db::DatabaseConnectionInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IterateSource {
    Database(DatabaseConnectionInfo),
    KvStore,
    Directory(String),
    //EmailInbox(ImapConnectionDetails)
}

pub trait IterateTaskKit {
    fn count(&self) -> usize;
    //fn iter(&mut self) -> Box<dyn Iterator>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IterateTask { }

impl IterateTask {


    //pub fn mysql(dbname: &str, user: &str, password: &str, host: &str, port: u16) -> Box<dyn IterateTaskKit> {

}


