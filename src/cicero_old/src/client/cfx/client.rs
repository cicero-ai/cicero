
use std::net::{TcpStream, ToSocketAddrs};
use std::io::{Read, Write};
use std::time::Duration;
use crate::error::Error;
use log::{info, debug, trace};
use super::ClientAuthenticator;
use crate::CLIENT_CONFIG;

pub struct CfxClient {
    stream: TcpStream
}

impl CfxClient {

    pub fn new() -> Result<Self, Error> {

        // Get address
        let hostname = format!("{}:{}", CLIENT_CONFIG.daemons.apollo.0, CLIENT_CONFIG.daemons.apollo.1);
        let mut address = match hostname.to_socket_addrs() {
            Ok(r) => r,
            Err(e) => return Err(Error::Generic("Invalid hostname within /config/client.yml file for Apollo server".to_string()))
        };
        let addr = address.next().unwrap();

        // Connect to apollo
        let mut stream = match TcpStream::connect_timeout(&addr, Duration::from_secs(5)) {
            Ok(r) => r,
            Err(e) => return Err(Error::ApolloConnectTimeout)
        };
        stream.set_read_timeout(Some(Duration::from_secs(5)));

        let mut client = Self {
            stream
        };
        trace!("Connected to CFX server, starting authentication");

        // Authenticate
        let mut authenticator = ClientAuthenticator::new(&mut client);
        authenticator.authenticate()?;
        info!("Successfully authenticated with Apollo server, ready for connections...");

        Ok(client)
    }

    /// Write line, then read next line and return
    pub fn write_read_line(&mut self, message: &[u8]) -> Result<Vec<u8>, Error> {

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

}


