
pub use self::api_client::ApiClient;
pub use self::mistral::Mistral;
pub use self::openai::OpenAI;
pub use self::claude::Claude;
pub use self::generic::Generic;
use crate::error::Error;
use crate::server::CONFIG;

pub mod api_client;
pub mod api_selector;
mod claude;
mod generic;
mod mistral;
mod openai;

trait ApiAdapter {
    fn get_api_key(&self) -> Result<String, Error>;
    fn get_chat_model(&self) -> String;
    fn get_embed_model(&self) -> String;
    fn get_chat_url(&self) -> String;
    fn get_embed_url(&self) -> String;
    fn get_http_headers(&self) -> Vec<String>;
}

/// Get appropriate API client depending on configuration
pub fn get_client() -> Result<ApiClient, Error> {

    let parts: Vec<String> = CONFIG.general.api_client.split(":").map(|w| w.to_string()).collect::<Vec<String>>();
    if parts[0] != "api" {
        return Err(Error::Generic("No API client configured on this server".to_string()));
    }

    let adapter: Box<dyn ApiAdapter> = match parts[1].as_str() {
        "mistral" => Box::new(Mistral::new(&parts[2].as_str())),
        "claude" => Box::new(Claude::new(&parts[2].as_str())),
        "openai" => Box::new(OpenAI::new(&parts[2].as_str())),
        "generic" => Box::new(Generic::new(&parts[2..])),
        _ => return Err( Error::Generic( format!("Unsupported APi provider type, {:?}", CONFIG.general.api_client) ))
    };

    Ok(ApiClient::new(adapter))
}


