use tokio::net::{TcpStream, TcpListener};
use tokio_tungstenite::{accept_hdr_async, WebSocketStream};
use tungstenite::handshake::server::{Request, Response};
use futures_util::{StreamExt, SinkExt};
use crate::user::User;
use crate::Error;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tungstenite::Message;
use uuid::Uuid;
use tokio::sync::mpsc::{self, Sender};
use falcon_cli::*;

#[derive(Clone)]
pub struct CfxServer {
    pub connections: Arc<Mutex<HashMap<Uuid, Sender<Message>>>>,
}

impl CfxServer {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    // Push a message to a user's WebSocket
    pub async fn push(&self, uuid: &Uuid, msg: String) -> Result<(), Error> {
        let connections = self.connections.lock().unwrap();
        if let Some(sender) = connections.get(uuid) {
            sender.send(Message::Text(msg.into())).await
                .map_err(|e| Error::Generic(format!("Push failed: {}", e)))?;
        }
        Ok(())
    }

    pub async fn handle_websocket(ws_stream: WebSocketStream<TcpStream>, peer_addr: std::net::SocketAddr, uuid: Uuid, connections: Arc<Mutex<HashMap<Uuid, Sender<Message>>>>) {
        let (mut sender, mut receiver) = ws_stream.split();
        let (tx, mut rx) = mpsc::channel::<Message>(100); // Buffer for sending

        // Add to pool
        connections.lock().unwrap().insert(uuid, tx.clone());

        // Send loop
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if let Err(e) = sender.send(msg).await {
                    println!("Send error: {}", e);
                    break;
                }
            }
        });

        // Receive loop
        while let Some(msg) = receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    println!("Received from {}: {}", peer_addr, text);
                    // Echo or handle—e.g., CFX commands
                    tx.send(Message::Text(format!("Echo: {}", text).into())).await.unwrap();
                }
                Ok(Message::Close(_)) => {
                    println!("Client {} disconnected", peer_addr);
                    connections.lock().unwrap().remove(&uuid);
                    break;
                }
                Err(e) => {
                    println!("Receive error: {}", e);
                    connections.lock().unwrap().remove(&uuid);
                    break;
                }
                _ => {}
            }
        }
    }
}


