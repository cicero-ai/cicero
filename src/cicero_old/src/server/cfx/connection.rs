
use std::net::TcpStream;
use std::io::{Read, Write};
use crate::error::Error;
use uuid::Uuid;
use std::time::Duration;
use chrono::{DateTime, Utc};
use super::CfxServerAuthenticator;
use log::info;

pub struct Connection {
    pub stream: TcpStream
}

pub struct AuthConnection {
    conn: Connection,
    uuid: Uuid,
    login_time: DateTime<Utc>
}

impl Connection {

    pub fn new(stream: TcpStream) -> Self {
        stream.set_read_timeout(Some(Duration::from_secs(5)));
        Self {
            stream
        }
    }

    /// Write line, then read next line and return
    pub fn write_read_line(&mut self, message: &Vec<u8>) -> Result<Vec<u8>, Error> {

        // Write line
        match self.stream.write_all(&message) {
            Ok(_) => { },
            Err(e) => {
                info!("Unable to write to cfx stream, error: {}", e.to_string());
                return Err(Error::Generic("Unable to write to CFX socket, broken connection?".to_string()));
            }
        };

        // Read next line and return
        self.read_line()
    }

    /// Read next line as bytes
    pub fn read_line(&mut self) -> Result<Vec<u8>, Error> {

        let mut buffer: [u8; 1024] = [0; 1024];
    let n = match self.stream.read(&mut buffer) {
            Ok(n) => n,
            Err(e) => {
                info!("Connection dropped, error: {}", e.to_string());
                return Err(Error::Generic("Unable to read from CFX socket, broken connection?".to_string()));
            }
        };

        // Check bytes read
        if n == 0 {
            return Err(Error::Generic("Unable to read from CFX socket, broken connection?".to_string()));
        }

        Ok(buffer[0..n].to_vec())
    }

    /// Read line as string
    pub fn read_line_str(&mut self) -> Result<String, Error> {
        let buf = self.read_line()?;
        let res_str = String::from_utf8_lossy(&buf).to_string();
        Ok(res_str)
    }

    /// Give error to client
    pub fn write_error(&mut self, msg: &str) {
        let mut message = vec![0x05];
            message.extend_from_slice(&msg.as_bytes());
            self.stream.write_all(&message).unwrap();
    }



}

impl AuthConnection {

    pub fn new(uuid: &Uuid, conn: Connection) -> Self {
        Self {
            conn,
            uuid: uuid.clone(),
            login_time: Utc::now()
        }
    }

}

