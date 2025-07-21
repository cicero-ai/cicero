
use std::collections::HashMap;
use uuid::Uuid;
use std::net::TcpStream;
use tokio::runtime::Runtime;
use std::sync::Arc;
use std::io::{Read, Write};
use super::{Connection, AuthConnection};

pub struct CfxServer {
    conn_id: i16,
    pool: HashMap<i16, AuthConnection>,
    rt: Runtime,
}

impl CfxServer {

    pub fn new() -> Self {

        Self {
            conn_id: 0,
            pool: HashMap::new(),
            rt: Runtime::new().unwrap()
        }

    }

    /// Check if stream has correct header bytes to be a CFX message or stnadard HTTP request
    pub fn is_cfx_stream(&self, stream: &mut TcpStream) -> bool {

        // Check first byte
        let mut buf = [0u8; 3];
        match stream.peek(&mut buf) {
            Ok(_) => { },
            Err(_) => return false
        };
        if buf[0] != 0x43 || buf[1] != 0x46 || buf[2] != 0x58 {
            return false;
        }

        true
    }

    /// Add authenticated connection to pool
    pub fn add_connection(&mut self, uuid: &Uuid, conn: Connection) {
        let auth_conn = AuthConnection::new(&uuid, conn);
        self.conn_id += 1;
        self.pool.insert(self.conn_id.clone(), auth_conn);
    }

}


