
use tokio::runtime::Runtime;
use lazy_static::lazy_static;
use self::apollo::ApolloServer;
use self::echo::EchoServer;
use std::env;
use crate::utils::sys;
use crate::api::CiceroAPI;
use crate::server::config::CiceroServerConfig;
use crate::CLIENT_CONFIG;
use tch::Device;

use crate::llm::models::ModelLibrary;

lazy_static! {
    pub static ref CONFIG: CiceroServerConfig = CiceroServerConfig::new();
    pub static ref MODELS: ModelLibrary = ModelLibrary::new();
    pub static ref INFERENCE_DEVICE: Device = sys::get_inference_device();
}

pub mod api;
pub mod apollo;
pub mod cfx;
pub mod config;
pub mod echo;
pub mod security;

/// Start server
pub fn start() {

    // Get flags
    let (server_type, port) = get_flags();

    // Start necessary server
    if server_type == "apollo".to_string() {
        let mut server = ApolloServer::new();
        server.start(&port);

    } else if server_type == "echo".to_string() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let mut echo = EchoServer::new();
            echo.start(&CLIENT_CONFIG.daemons.echo.1.clone()).await.unwrap();
        });
    }

}

/// Collect flags from command line
fn get_flags() -> (String, u16) {

    let (mut server_type, mut port) = (String::new(), 0);
    let mut args = env::args().collect::<Vec<String>>();
        args.remove(0);

    while !args.is_empty() {
        let chk_arg = args[0].to_string();
        args.remove(0);

        if chk_arg == "-t" {
            server_type = args[0].to_string();
            args.remove(0);
        } else if chk_arg == "-p" {
            port = args[0].parse::<u16>().unwrap();
            args.remove(0);
        }
    }

    // Default port
    if server_type == "apollo".to_string() && port == 0 {
        port = CONFIG.daemons.apollo.1;
    }

    (server_type.clone(), port.clone())
}




