
use serde_derive::{Serialize, Deserialize};
use uuid::Uuid;
use std::time::Instant;
use chrono::{DateTime, Utc};
pub use self::user_key::UserKey;
pub use self::postman::Postman;

pub mod forge;
pub mod postman;
pub mod user_key;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role {
    admin,
    user,
    guest
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApolloApiKey {
    pub key: String,
    pub uuid: Option<Uuid>,
    pub creation_time: DateTime<Utc>,
    pub last_seen: DateTime<Utc>
}



