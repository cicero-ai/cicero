

use atlas_http::{HttpRequest, HttpResponse, HttpBody};
use falcon_cli::*;
use tokio::net::{TcpStream, TcpListener};
use tungstenite::handshake::server::{Request, Response};
use tokio_tungstenite::{accept_hdr_async, WebSocketStream};
use uuid::Uuid;
use rustls::server::ServerConfig;
use rustls_pemfile::{certs, pkcs8_private_keys};
use rustls::pki_types::pem::SectionKind::PrivateKey;
use rustls::pki_types::{PrivateKeyDer, CertificateDer};
use rustls::pki_types::pem::SectionKind::Certificate as RustlsCert;
use tokio_rustls::{TlsAcceptor, server::TlsStream};
use cicero::utils::sys;
use std::{io, fs};
use std::io::BufReader;
use std::path::PathBuf;
use std::io::Write;
use std::fs::File;
use crate::llm::chat::ChatRouter;
use crate::cfx::CfxServer;
use crate::user::User;
use std::sync::{Arc, Mutex};
use super::{Vault, Authenticator, AuthUser};
use crate::{Error, CONFIG};
use log::{info, debug, error};
use tokio::time::Duration;


#[derive(Clone)]
pub struct ApolloServer {
    vault: Vault,
    armor: Authenticator,
    chat_router: ChatRouter,
    cfx: CfxServer,
}

impl ApolloServer {
    pub fn new() -> Result<Self, Error> {
        let server = Self {
            vault: Vault::open()?,
            armor: Authenticator::new(),
            chat_router: ChatRouter::new(),
            cfx: CfxServer::new(),
        };
        Ok(server)
    }

    pub async fn start(&mut self) -> Result<(), Error> {

        // Start HTTPS server
        self.start_https().await?;

        // Start cfx (web socket)
        self.start_cfx().await?;

        cli_info!("Listening on ports {} and {}... press Ctrl+C to quit.", CONFIG.daemon.https_port, CONFIG.daemon.ws_port);
        tokio::signal::ctrl_c().await?;
        Ok(())
    }

    async fn start_https(&mut self) -> Result<(), Error> {

        // Load SSL certs
        let datadir = sys::get_datadir();
        let cert_file = File::open(format!("{}/manager/ssl/cicero.pem", datadir))?;
        let key_file = File::open(format!("{}/manager/ssl/privkey.pem", datadir))?;
        let mut cert_reader = BufReader::new(cert_file);
        let mut key_reader = BufReader::new(key_file);

        // Load certificates
        let certs: Vec<CertificateDer<'static>> = certs(&mut cert_reader)
            .map(|cert| cert.unwrap())
            .collect();

        // Load private key
        let key = pkcs8_private_keys(&mut key_reader)
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "No private key found"))?;
        let privkey = PrivateKeyDer::Pkcs8(key?);

        // Rustls config for REST API over HTTPS
        let config = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(certs, privkey)
            .map_err(|e| Error::IO(e.to_string()))?;
        let acceptor = TlsAcceptor::from(Arc::new(config));

        // HTTPS listener
        let https_addr = format!("{}:{}", CONFIG.daemon.host, CONFIG.daemon.https_port);
        let https_listener = TcpListener::bind(&https_addr).await
            .map_err(|e| Error::Apollo(format!("Unable to bind to HTTPS address {}, error {}", https_addr, e)))?;
        cli_info!("Started Apollo HTTPS server, listening on {}...", https_addr);

        // Clone variables for async
        let https_vault = Arc::new(self.vault.clone());
        let https_armor = self.armor.clone();

        // HTTPS server
        let https_acceptor = acceptor.clone();
        tokio::spawn(async move {
            loop {
                match https_listener.accept().await {
                    Ok((stream, peer_addr)) => {
                        let acceptor = https_acceptor.clone();
                        let vault = https_vault.clone();
                        let armor = https_armor.clone();
                        tokio::spawn(async move {
                            if let Ok(mut tls_stream) = acceptor.accept(stream).await {
                                let mut has_output = false;

                                // Authenticate request
                                if let Ok(http_req) = HttpRequest::build_async_tls(&mut tls_stream).await {
                                    if let Ok(Some(user)) = armor.check_http_req(&http_req, &vault) {
                                        handle(http_req, Some(user), &mut tls_stream, peer_addr).await;
                                        has_output = true;
                                    }
                                }

                                if !has_output {
                                    send_401(&mut tls_stream, peer_addr).await;
                                }
                            }
                        });
                    }
                    Err(e) => {
                        cli_error!("Failed to accept HTTPS connection: {}", e);
                        tokio::time::sleep(Duration::from_millis(100)).await;
                    }
                }
            }
        });

        Ok(())
    }

    /// Start CFX server (web sockets)
    async fn start_cfx(&mut self) -> Result<(), Error> {

        // WebSocket listener
        let ws_addr = format!("{}:{}", CONFIG.daemon.host, CONFIG.daemon.ws_port);
        let ws_listener = TcpListener::bind(&ws_addr).await
            .map_err(|e| Error::Apollo(format!("Unable to bind to web socket address {}, error {}", ws_addr, e)))?;
        cli_info!("Started Apollo web socket server, listening on {}...", ws_addr);


        // Clone variables for async
        let ws_vault = Arc::new(self.vault.clone());
        let ws_armor = Arc::new(self.armor.clone());
        let ws_connections = self.cfx.connections.clone();

        tokio::spawn(async move {
            // Main WebSocket listener loop
            loop {
                // Accept incoming WebSocket connections
                let accept_result = ws_listener.accept().await;
                match accept_result {
                    Ok((stream, peer_addr)) => {
                        println!("New WebSocket connection from {}", peer_addr);

                        // Clone shared resources for this connection
                        let vault_ref = ws_vault.clone();
                        let armor_ref = ws_armor.clone();
                        let connections_ref = ws_connections.clone();

                        // Spawn a task to handle this WebSocket connection
                        tokio::spawn(async move {
                            // Variable to store authenticated user's UUID
                            let mut authenticated_uuid: Option<Uuid> = None;

                            // Define callback for WebSocket handshake authentication
                            let auth_callback = |request: &Request, response: Response| {
                                // Extract API key from headers, default to empty if missing
                                let api_key = request.headers()
                                    .get("X-API-Key")
                                    .and_then(|val| val.to_str().ok())
                                    .unwrap_or("");

                                // Authenticate the request using the API key
                                let auth_result = armor_ref.check_api_key(api_key, &vault_ref);
                                match auth_result {
                                    Ok(Some(user)) => {
                                        // Authentication succeeded—stash UUID
                                        authenticated_uuid = Some(user.lock().unwrap().uuid);
                                        Ok(response)
                                    }
                                    Ok(None) | Err(_) => {
                                        // Authentication failed—build 401 response
                                        let unauthorized_response = Response::builder()
                                            .status(tungstenite::http::StatusCode::UNAUTHORIZED)
                                            .body(Some("Unauthorized".to_string()))
                                            .unwrap();
                                        Err(unauthorized_response)
                                    }
                                }
                            };

                            // Attempt to upgrade the TCP stream to a WebSocket connection
                            let upgrade_result = accept_hdr_async(stream, auth_callback).await;
                            match upgrade_result {
                                Ok(websocket_stream) => {
                                    // Get the UUID from authentication—panic if missing (shouldn’t happen)
                                    let uuid = authenticated_uuid.expect("UUID should be set after successful auth");
                                    println!("WebSocket upgraded for peer {} with UUID {}", peer_addr, uuid);

                                    // Hand off to CfxServer for WebSocket handling
                                    CfxServer::handle_websocket(websocket_stream, peer_addr, uuid, connections_ref).await;
                                }
                                Err(error) => {
                                    println!("WebSocket auth/upgrade failed for {}: {}", peer_addr, error);
                                }
                            }
                        });
                    }
                    Err(error) => {
                        println!("Failed to accept WebSocket connection: {}", error);
                    }
                }
            }
        });

        Ok(())
    }

}

async fn handle(_http_req: HttpRequest, _user: AuthUser, _stream: &mut TlsStream<TcpStream>, peer_addr: std::net::SocketAddr) {
    cli_info!("Handled request from {}", peer_addr);
    // Placeholder—your logic here
}

async fn send_401(_stream: &mut TlsStream<TcpStream>, peer_addr: std::net::SocketAddr) {
    cli_info!("Unauthorized request from {}", peer_addr);
    // Placeholder—send 401 response
}

// Push example
async fn notify_user(server: &ApolloServer, uuid: Uuid, msg: &str) -> Result<(), Error> {
    server.cfx.push(&uuid, msg.to_string()).await
}




