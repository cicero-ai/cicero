
use tokio_tungstenite::{connect_async, WebSocketStream, MaybeTlsStream};
use tungstenite::Message;
use futures_util::{StreamExt, SinkExt};
use crate::{Error, CONFIG};
use std::net::SocketAddr;

pub struct CfxClient {
    ws_stream: WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>,
    api_key: String,
}

impl CfxClient {
    pub async fn connect(ws_addr: &str) -> Result<Self, Error> {
        // Grab API key from CONFIG.local_user—guaranteed not None
        let api_key = CONFIG.local_user
            .as_ref()
            .expect("CONFIG.local_user should not be None")
            .api_key
            .clone();

        // Build WebSocket URL with API key in header
        let url = format!("ws://{}", ws_addr);
        let request = tungstenite::handshake::client::Request::builder()
            .uri(&url)
            .header("X-API-Key", &api_key)
            .build()?;

        // Connect and check upgrade
        let (ws_stream, response) = connect_async(request).await
            .map_err(|e| Error::Generic(format!("WebSocket connect failed: {}", e)))?;

        // Verify upgrade—101 Switching Protocols
        if response.status() != tungstenite::http::StatusCode::SWITCHING_PROTOCOLS {
            return Err(Error::Generic(format!(
                "WebSocket upgrade failed: got status {}", 
                response.status()
            )));
        }

        println!("Connected to WebSocket at {}", ws_addr);
        Ok(Self { ws_stream, api_key })
    }

    pub async fn send(&mut self, msg: &str) -> Result<(), Error> {
        self.ws_stream
            .send(Message::Text(msg.to_string()))
            .await
            .map_err(|e| Error::Generic(format!("Failed to send WebSocket message: {}", e)))?;
        println!("Sent message: {}", msg);
        Ok(())
    }

    pub async fn on_message<F>(&mut self, mut callback: F) -> Result<(), Error> 
    where
        F: FnMut(String),
    {
        while let Some(msg) = self.ws_stream.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    println!("Received message: {}", text);
                    callback(text);
                }
                Ok(Message::Close(_)) => {
                    println!("WebSocket connection closed by server");
                    break;
                }
                Err(e) => {
                    return Err(Error::Generic(format!("WebSocket error: {}", e)));
                }
                _ => {} // Ignore ping/pong, binary for now
            }
        }
        Ok(())
    }
}


