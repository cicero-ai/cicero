
use serde_derive::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use uuid::Uuid;
use crate::server::security::{ApolloApiKey, UserKey};
use crate::llm::chat::Conversation;
use crate::llm::FaissIndex;
use cicero_sdk::chat::{ChatPipeline, ChatKit};
use crate::utils::sys;
use crate::server::cfx::CfxStorage;
use crate::error::Error;
use std::fs;
use log::error;
use x25519_dalek::PublicKey;
use super::{schema, UserSqlDb};

#[derive(Serialize, Deserialize)]
pub struct ServerUser {
    pub uuid: Uuid,
    pub name: String,
    pub nickname: String,
    pub public_key: PublicKey,
    pub email: String,
    pub conversation_pipeline: String,
    pub active_conversation: Uuid,
    pub conversations: HashMap<Uuid, Conversation>,
    pub apollo_api_key: String,
    pub installed_plugins: Vec<String>,
    #[serde(skip_serializing, skip_deserializing)]
    pub faiss: FaissIndex,
    pub creation_time: DateTime<Utc>,
    pub last_seen: DateTime<Utc>
}

impl ServerUser {

    /// Create new user on server
    pub fn create(
        uuid: &Uuid,
        name: &str,
        email: &str,
        public_key: &PublicKey,
        api_key: &String,
        cicero_key: &UserKey
    ) -> Result<ServerUser, Error> {

        // Create directory
        let faiss_datadir = format!("{}/profiles/{}/faiss/profile", sys::get_datadir(), uuid.to_string()); 
        sys::prepare_parent_dir(&faiss_datadir.as_str());
 
        // Get nickname
        let mut nickname = name.clone();
        if let Some(index) = name.find(" ") {
            nickname = &name[0..index];
        }

        // Start conversation
        let conversation = Conversation::new(&uuid, &nickname);
        let active_conversation = conversation.id.clone();
    let mut conversations: HashMap<Uuid, Conversation> = HashMap::new();
        conversations.insert(conversation.id.clone(), conversation);

        let user = ServerUser {
            uuid: uuid.clone(),
            name: name.to_string(),
            nickname: nickname.to_string(),
            email: email.to_string(),
            conversation_pipeline: "core.introduction".to_string(),
            active_conversation,
            conversations,
            public_key: public_key.clone(),
            apollo_api_key: api_key.clone(),
            installed_plugins: vec!["core".to_string()],
            faiss: FaissIndex::new(&faiss_datadir.as_str(), &cicero_key, &public_key),
            creation_time: Utc::now(),
            last_seen: Utc::now()
        };

        // Open storage items
        user.get_storage(&cicero_key);
        let sqldb = user.get_sqldb();

        // Create sql database schema
        match schema::create(&sqldb) {
            Ok(_) => { },
            Err(e) => return Err(Error::Generic(e.to_string()))
        };

        Ok(user)
    }

    /// Get user's storage
    pub fn get_storage(&self, cicero_key: &UserKey) -> CfxStorage {
        let storage_dir = format!("{}/profiles/{}/data", sys::get_datadir(), self.uuid.to_string());
        CfxStorage::new(&storage_dir.as_str(), &cicero_key, &self.public_key)
    }

    /// Get sql database
    pub fn get_sqldb(&self) -> UserSqlDb {
        let dbfile = format!("{}/profiles/{}/user.db", sys::get_datadir(), self.uuid.to_string());
        UserSqlDb::new(&dbfile)
    }

    /// Get active conversatoin
    pub fn conv(&mut self) -> &mut Conversation {
        self.conversations.get_mut(&self.active_conversation).unwrap()
    }

}


