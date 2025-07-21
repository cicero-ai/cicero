
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use opus::db::OpusSqlDb;
use opus::preludes::*;
use std::collections::HashMap;
use uuid::Uuid;
use crate::apollo::ApolloApiKey;
use cicero::security::UserKey;
use crate::llm::chat::Conversation;
use crate::llm::rag::FaissIndex;
use cicero_sdk::chat::{ChatPipeline, ChatKit};
use cicero::utils::sys;
use crate::cfx::CfxStorage;
use crate::Error;
use std::fs;
use log::error;
use cicero::preludes::*;
use x25519_dalek::PublicKey;
use super::schema;

#[derive(Serialize, Deserialize)]
pub struct User {
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
    #[serde(skip)]
    pub faiss: FaissIndex,
    pub creation_time: DateTime<Utc>,
    pub last_seen: DateTime<Utc>
}

pub struct UserThreadSafe {
    pub uuid: Uuid,
    pub name: String,
    pub nickname: String,
    pub email: String,
    pub public_key: PublicKey,
    pub conversation_pipeline: String,
    pub active_conversation: Uuid,
    pub installed_plugins: Vec<String>,
    pub creation_time: DateTime<Utc>,
    pub last_seen: DateTime<Utc>
}


impl User {
    pub fn create(
        uuid: &Uuid,
        name: &str,
        email: &str,
        public_key: &PublicKey,
        api_key: &String,
        cicero_key: &UserKey
    ) -> Result<Self, Error> {

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

        let user = Self {
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
            faiss: FaissIndex::new(&faiss_datadir.as_str(), &cicero_key, &public_key).map_err(|e| Error::Faiss(e.to_string()) )?,
            creation_time: Utc::now(),
            last_seen: Utc::now()
        };

        // Open storage items
        user.get_storage(&cicero_key);

        // Create sql database schema
        let sqldb = user.open_sqldb()?;
        schema::create(&sqldb)
            .map_err(|e| Error::Cfx(e.to_string()) )?;

        Ok(user)
    }

    /// Get user's storage
    pub fn get_storage(&self, cicero_key: &UserKey) -> Result<CfxStorage, Error> {
        let storage_dir = format!("{}/profiles/{}/data", sys::get_datadir(), self.uuid.to_string());
        CfxStorage::new(&storage_dir.as_str(), &cicero_key, &self.public_key)
    }

    /// Get sql database
    pub fn open_sqldb(&self) -> Result<OpusSqlDb<Sqlite>, Error> {
        let dbfile = format!("{}/profiles/{}/user.db", sys::get_datadir(), self.uuid.to_string());
        OpusSqlDb::connect_sqlite(&dbfile)
            .map_err(|e| Error::Cfx( format!("Unable to load SQLite database for user, error: {}", e)) )
    }

    /// Get active conversation
    pub fn conv(&mut self) -> &mut Conversation {
        self.conversations.get_mut(&self.active_conversation).unwrap()
    }

    /// To thread safe user
    fn to_thread_safe(&self) -> UserThreadSafe {
        UserThreadSafe {
            uuid: self.uuid.clone(),
            name: self.name.to_string(),
            nickname: self.nickname.to_string(),
            email: self.email.to_string(),
            public_key: self.public_key.clone(),
            conversation_pipeline: self.conversation_pipeline.to_string(),
            active_conversation: self.active_conversation.clone(),
            installed_plugins: self.installed_plugins.clone(),
            creation_time: self.creation_time.clone(),
            last_seen: self.last_seen.clone()
        }
    }
}



