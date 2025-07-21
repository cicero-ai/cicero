
pub use self::chat_manager::ChatManager;
pub use self::echo_assistant::EchoAssistant;
pub use self::hardware::HardwareProfile;
pub use self::hq::CiceroHQ;
pub use self::manager::CiceroManager;
pub use self::plugin_manager::PluginManager;
pub use self::user_manager::UserManager;
pub use self::vault::Vault;

mod chat_manager;
mod echo_assistant;
pub mod hardware;
mod hq;
pub mod manager;
mod models;
mod ollama;
mod plugin_manager;
pub mod setup;
mod user_manager;
mod vault;

