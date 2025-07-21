
use serde::{Serialize, Deserialize};

pub use openssl;
pub use x25519_dalek;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub status: String,
    pub message: String,
    pub data: T
}


