

use x25519_dalek::PublicKey;
use uuid::Uuid;
use std::io::Write;
use crate::server::security::{forge, Postman, UserKey};
use crate::error::Error;
use super::Connection;
use log::{debug, info, trace};

pub struct CfxServerAuthenticator<'a> {
    conn: &'a mut Connection,
    challenge: [u8; 32],
    user_public_key: [u8; 32]
}

impl CfxServerAuthenticator<'_> {

    pub fn new(conn: &mut Connection) -> CfxServerAuthenticator {
        CfxServerAuthenticator { 
            conn,
            challenge: [0; 32],
            user_public_key: [0; 32]
        }
    }

        // Handshake
    pub fn handshake(&mut self) -> Result<Uuid, Error> {
        info!("Starting new CFX authentication");

        // Check heading
        self.heading()?;
        debug!("Auth header ok");

        // Greet user
        let uuid = self.greet()?;
        debug!("Got uuid from client, {}", uuid.to_string());

        Ok(uuid)
    }

    /// Check heading message
    fn heading(&mut self) -> Result<(), Error> {

        // Get first line
        let header = self.conn.read_line_str()?;

        // Check line
        if !header.ends_with(&"CFX/0.1".to_string()) {
            //self.conn.write_error("Invalid greeting");
            //return Err(Error::Generic("Invalid CFX header supplied".to_string()));
        }

        Ok(())
    }

    /// Greet user, get uuid
    fn greet(&mut self) -> Result<Uuid, Error> {

        // Get next line
        let res: Vec<u8> = self.conn.write_read_line(&vec![0x02, 0x01])?;
        if res[0] != 0x02 || res[1] != 0x02 {
            self.conn.write_error("Invalid auth line");
            return Err(Error::Generic(format!("Did not receive valid response from client during greeting, got {:?} {:?}", res[0], res[1])));
        }
        self.challenge.copy_from_slice(&res[2..34]);

        // Get uuid
        let mut uuid_bytes: [u8; 16] = [0; 16];
        uuid_bytes.copy_from_slice(&res[34..50]);

        // Get public key bytes
        self.user_public_key.copy_from_slice(&res[50..82]);

        // Parse uuid
        let uuid = Uuid::from_bytes(uuid_bytes);
        Ok(uuid.clone())
    }

    /// Complete authentication
    pub fn authenticate(&mut self, uuid: &Uuid, apollo_key: &UserKey) -> Result<PublicKey, Error> {
        debug!("Completing CFX authentication, verifying server response");

        // Encrypt message
        let client_challenge: [u8; 32] = forge::get_nonce(None);
        let message = [self.challenge.clone(), client_challenge.clone()].concat();
        let public_key = PublicKey::from(self.user_public_key);

        // Encrypt message
        let mut postman = Postman::new(&apollo_key, &public_key);
        let encrypted_message = postman.encrypt(&message);

        // Get auth line
        let mut auth_line = vec![0x02, 0x03];
        auth_line.extend_from_slice(&encrypted_message);

        // Send auth line to client
        let res: Vec<u8> = self.conn.write_read_line(&auth_line)?;
        if res[0] != 0x02 || res[1] != 0x04 {
            self.conn.write_error("Invalid verification format");
            return Err(Error::Generic(format!("Did not receive valid response from client during verification, got {:?} {:?}", res[0], res[1])));
        }

        //Decrypt
        let decrypted_res = postman.decrypt(&res[2..])?;
        if decrypted_res != client_challenge {
            self.conn.write_error("Auth verification failed.");
            return Err(Error::Generic("Invalid auth, client did not successfully complete auth challenge.".to_string()));
        }

        // Return public key for login
        Ok(public_key)
    }

    // Copmlete authentication, grant access
    pub fn grant(&mut self) {
        self.conn.stream.write_all(&vec![0x02, 0x05]).unwrap();
        info!("Successfully completed CFX authentication for client, adding to connection pool.");
    }

    /// Deny authentication, login failed
    pub fn deny(&mut self) {
        self.conn.write_error("Auth failed, invalid encryption key.");
    }
}

