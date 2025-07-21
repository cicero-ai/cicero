
use serde::{Serialize, Deserialize};
use std::hash::{Hash, Hasher};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use tokio::runtime::Runtime;
use crate::user::UserThreadSafe;
use std::sync::{Arc, Mutex};
use falcon_cli::*;
use log::error;

pub type AuthUser = Option<Arc<Mutex<UserThreadSafe>>>;

//pub use self::api::ApiResponse;
pub use self::auth::Authenticator;
pub use self::config::{CiceroServerConfig, NetworkMode, ServerMode};
//pub use self::router::{HttpRouter, HttpMethod};
pub use self::server::ApolloServer;
pub use self::vault::Vault;


//mod api;
mod auth;
mod config;
mod hq;
//mod router;
pub mod setup;
mod server;
mod vault;

/// Start Apollo server daemon
pub fn start_server() {
    let mut server = match ApolloServer::new() {
        Ok(r) => r,
        Err(e) => { 
            cli_error!("Unable to start Apollo server, error: {}", e);
            std::process::exit(1);
        }
    };

    let mut rt = match Runtime::new() {
        Ok(r) => r,
        Err(e) => {
            cli_error!("Unable to initialize tokio runtime, error: {}", e);
            return;
        }
    };





    if let Err(e) = rt.block_on(server.start()) {
        cli_error!("Unable to start Apollo server, error: {}", e);
    }

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApolloApiKey {
    pub key: String,
    pub uuid: Option<Uuid>,
    pub creation_time: DateTime<Utc>,
    pub last_seen: DateTime<Utc>
}

impl Hash for ApolloApiKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}


