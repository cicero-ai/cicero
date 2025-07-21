
use crate::error::Error;
use crate::server::security::{forge, Postman};
use super::CfxClient;
use crate::CLIENT_CONFIG;
use log::{debug, error};

pub struct ClientAuthenticator<'a> {
    client: &'a mut CfxClient,
    server_challenge: [u8; 32]
}

impl ClientAuthenticator<'_> {

    pub fn new(client: &mut CfxClient) -> ClientAuthenticator {
        ClientAuthenticator { 
            client,
            server_challenge: forge::get_nonce(None)
        }
    }

    /// Authenticate with Apollo
    pub fn authenticate(&mut self) -> Result<(), Error>{

        // Greeting
        self.greeting()?;
        debug!("Completed greeting to CFX server");

        // Send uuid
        let res = self.send_uuid()?;
        debug!("Sent uuid to CFX server");

        // Complete authentication
        self.complete(&res)?;

        Ok(())
    }

    /// Send greeting
    fn greeting(&mut self) -> Result<(), Error> {

        // Send line
        let mut res: Vec<u8> = self.client.write_read_line("CFX/0.1".as_bytes())?;
        if res[0] != 0x02 || res[1] != 0x01 {
            let err_msg = String::from_utf8_lossy(&res).to_string();
            return Err(Error::Generic(format!("Invalid response from CFX server: {}", err_msg.trim())));
        }

        Ok(())
    }

    /// Auth - send uuid
    fn send_uuid(&mut self) -> Result<Vec<u8>, Error> {

        self.server_challenge = forge::get_nonce(None);
        let mut auth_line = vec![0x02, 0x02];
        auth_line.extend_from_slice(&self.server_challenge.clone());
        auth_line.extend_from_slice(&CLIENT_CONFIG.current_uuid.unwrap().into_bytes());
        auth_line.extend(CLIENT_CONFIG.current_user.clone().unwrap().security_key.public.to_bytes());

        // Send line
        let res: Vec<u8> = self.client.write_read_line(&auth_line)?;
        if res[0] != 0x02 || res[1] != 0x03 {
            let err_msg = String::from_utf8_lossy(&res).to_string();
            return Err(Error::Generic(format!("Invalid response from CFX server: {}", err_msg.trim())));
        }

        // Return
        Ok(res[2..].to_vec())
    }

    /// Complete authentication
    fn complete(&mut self, input: &[u8]) -> Result<(), Error> {
        debug!("Completing CFX authentication, msg length {}, porints: {:?} {:?} / {:?} {:?}", input.len(), input[0], input[1], input[input.len()-2], input[input.len()-1]);

        // Decrypt
        let tmp_user = CLIENT_CONFIG.current_user.clone().unwrap();
        let postman = Postman::new(&tmp_user.security_key, &tmp_user.cicero_public_key);
        let message = postman.decrypt(&input)?;

        // Verify challenge
        if (message[0..32] != self.server_challenge) {
            return Err(Error::Generic("Verification failed, invalid challenge answer received from server.".to_string()));
        }
        let enc_message = postman.encrypt(&message[32..64]);

        // Get message to send
        let mut res_message = vec![0x02, 0x04];
        res_message.extend(enc_message);

        // Send message
        let res = self.client.write_read_line(&res_message)?;
        if res[0] != 0x02 || res[1] != 0x05 {
            return Err(Error::Generic("Unable to authenticate, server did not provide valid response to challenge answer.".to_string()));
        }

        Ok(())
    }

}

