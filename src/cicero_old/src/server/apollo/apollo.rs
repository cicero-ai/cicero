
use crate::llm::services::{SentenceEmbeddings, Summarizer, POSTagging, TextGeneration, QuestionAnswer};
use crate::llm::chat::ChatRouter;
use crate::server::cfx::{CfxServer, Connection, CfxServerAuthenticator};
use serde::{Serialize, Deserialize};
use atlas_http::{HttpRequest, HttpResponse, HttpBody};
use url::Url;
use std::net::{TcpStream, TcpListener};
use std::io::{BufReader, BufRead, Write};
use crate::server::apollo::nlp;
use crate::server::apollo::user::ServerUser;
use crate::server::api;
use crate::error::Error;
use crate::server::apollo::manager::CiceroManager;
use log::{info, debug, error};
use crate::server::CONFIG;

pub struct ApolloServer {
    manager: CiceroManager,
    chat_router: ChatRouter,
    cfx: CfxServer,
    pub summarizer: Summarizer,
    pub text_generation: TextGeneration,
    pub question_answer: QuestionAnswer,
    pub pos_tagging: POSTagging,
    pub sentence_embeddings: SentenceEmbeddings
}

impl ApolloServer {

    pub fn new() -> Self {

        // Get services to autoload
        let autoload: Vec<&str> = CONFIG.models.autoload.split(",").map(|d| d.trim()).collect();
        info!("Initializing the Apollo daemon, please be patient as this may take a couple minutes to load necessary LLMs...");

        Self {
            manager: CiceroManager::new(),
            chat_router: ChatRouter::new(),
            cfx: CfxServer::new(),
            summarizer: Summarizer::new(Default::default(), autoload.contains(&"summarization")),
            text_generation: TextGeneration::new(Default::default(), autoload.contains(&"text_generation")),
            question_answer: QuestionAnswer::new(Default::default(), autoload.contains(&"question_answer")),
            pos_tagging: POSTagging::new(Default::default(), autoload.contains(&"pos_tagging")),
            sentence_embeddings: SentenceEmbeddings::new(Default::default(), autoload.contains(&"sentence_embeddings"))
        }
    }

    /// Start server
    pub fn start(&mut self, port: &u16) -> Result<(), Error> {

        // Start listening
        let address = format!("{}:{}", CONFIG.daemons.apollo.0, port);
        let listener = match TcpListener::bind(&address) {
            Ok(r) => r,
            Err(e) => {
                error!("Unable to start Apollo server, error: {}", e.to_string());
                std::process::exit(1);
            }
        };
        info!("Started Apollo server, listening for connections on {}...", address);

        // Get incoming connection
        loop {
            let (mut stream, _) = listener.accept().unwrap();

            // Check for CFX stream
        if self.cfx.is_cfx_stream(&mut stream) {
            self.handle_cfx(stream);
            continue;
        }

            // Handle request
            let res = self.handle(&mut stream);

            // Output response
            stream.write(res.raw().as_bytes());
        }

        Ok(())
    }

    ///  Handle cfx stream
    fn handle_cfx(&mut self, mut stream: TcpStream) -> Result<(), Error> {

        // Start new connection
        let mut conn = Connection::new(stream);
        let mut authenticator = CfxServerAuthenticator::new(&mut conn);

        // Handshake
        let uuid = match authenticator.handshake() {
            Ok(r) => r,
            Err(e) => {
                error!("Invalid CFX handshake: {}", e.to_string());
                return Err(e);
            }
        };

        // Check for uuid
        if !self.manager.vault.profiles.contains_key(&uuid) {
            error!("Provided UUID does not exist in database, {}", uuid.to_string());
            return Err(Error::Generic("Uuid does not exist in database".to_string()))
        }

        // Finish authentication
        let public_key = match authenticator.authenticate(&uuid, &self.manager.vault.chat_key) {
            Ok(r) => r,
            Err(e) => {
                error!("Invalid CFX auth completion: {}", e.to_string());
                return Err(e);
            }
        };

        // Login user
        if !self.manager.login(&uuid, &public_key) {
            authenticator.deny();
            error!("Invalid CFX authentication, unable to login to user's profile, vad public key.");
            return Err(Error::Generic("Invalid authentication, bad public key.".to_string()));
        }

        // Grant access
        authenticator.grant();
        drop(authenticator);

        // Add connection to pool
        self.cfx.add_connection(&uuid, conn);
        info!("Successfully authenticated client via CFX...");
        Ok(())
    }

    /// Handle a connection
    fn handle(&mut self, stream: &mut TcpStream) -> HttpResponse {

        // Get peer
        let peer = match stream.peer_addr() {
            Ok(r) => r,
            Err(e) => return api::response(400, "Invalid request, no peer.", String::new())
        };
        info!("Received connection from {}.", peer.ip());

        // Get http request
        let req = match HttpRequest::build(stream) {
            Ok(r) => r,
            Err(e) => return api::response(400, "Either malformed request, or pipe interrupted during transfer.  Did not receive full request.", String::new())
        };

        // Route request as needed
        let url = Url::parse(&req.url).unwrap();
        let res = self.route(&url.path().trim_start_matches("/").trim_end_matches("/"), &req, stream);
        res
    }

    /// Route request as needed
    pub     fn route(&mut self, path: &str, req: &HttpRequest, stream: &mut TcpStream) -> HttpResponse {
        debug!("Routing request to '{}'", path);

        // Check version
        let parts: Vec<String> = path.split("/").map(|p| p.to_string()).collect();
        if parts.len() < 3 {
            return api::response(404, "No endpoint here", String::new());
        } else if parts[0] != "v1" {
            return api::response(400, "Invalid version", String::new());
        }

        // Perform action
        let res = match parts[1].as_str() {
            "nlp" => nlp::handle(self, &parts, &req),
            _ => self.manager.handle_v1_api_request(&parts, &req, stream),
        };

        res
    }

}

