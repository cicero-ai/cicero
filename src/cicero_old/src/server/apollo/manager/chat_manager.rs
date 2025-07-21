
use uuid::Uuid;
use std:: collections::HashMap;
use serde_derive::{Serialize, Deserialize};
use atlas_http::{HttpResponse, HttpRequest};
use cicero_core::chat::IntroductionChat;
use tokio::runtime::Runtime;
use std::sync::{Arc, Mutex};
use std::io::Write;
use std::net::TcpStream;
use crate::server::apollo::user::ServerUser;
use crate::llm::chat::{Conversation, ChatMessage, UserReply, ChatRouter};
use crate::server::api;
use cicero_sdk::chat::{{ChatKit, ChatPipeline}};
use super::Vault;
use log::{info, debug, error};
use crate::CLIENT_CONFIG;

pub struct ChatManager {
    conversations: HashMap<Uuid, Conversation>
}

impl ChatManager {

    pub fn new() -> Self {
        Self {
            conversations: HashMap::new()
        }
    }

    /// Check in
    pub fn checkin(&mut self, auth_user: Option<Arc<Mutex<ServerUser>>>) -> HttpResponse {
        debug!("Initiating chat check-in");

        /// Check if logged in
        if auth_user.is_none() {
            debug!("No user authenticated, skipping chat check-in");
            return api::response(200, "", String::new());
        }
        let mut user = auth_user.as_ref().unwrap().lock().unwrap();

        // Get active conversation
        user.conv().reset_context();
        let nickname = user.nickname.to_string();

        // Check conversation pipeline
        if user.conversation_pipeline == "core.introduction".to_string() {
            debug!("Starting introductory chat");
            user.conv().kit = Some(ChatKit::new(Box::new(IntroductionChat::new())));
        }

        let prompt = user.conv().format_prompt();
        api::response(200, "", prompt)
    }

    /// User reply
    pub fn user_reply(&mut self, auth_user: Option<Arc<Mutex<ServerUser>>>, req: &HttpRequest, stream: &mut TcpStream) -> HttpResponse {

        // Get user
        if auth_user.is_none() {
            return api::response(200, "", String::from("Please login to reply."));
        }
        //let binding_user = auth_user.unwrap();
        let mut user = auth_user.as_ref().unwrap().lock().unwrap();

        let conv_id = user.active_conversation.clone();
        debug!("Received chat reply from user, {}", user.nickname);

        // Get conversation
        let mut conv: &mut Conversation = match user.conversations.get_mut(&conv_id) {
            Some(r) => r,
            None => return api::response(200, "", String::from("An error occured, no active conversation found within profile."))
        };

        // Add user message to conversation
        let message = req.body.params().get("message").unwrap().clone();
        conv.add_user(auth_user.clone(), &message);

        // Route reply to LLM for response generation
        let mut router = ChatRouter::new();
        let rt = Runtime::new().unwrap();
        let response = rt.block_on(async {
            router.route_streamed(&conv, stream).await
        });
        conv.add_assistant(&response);

        // Get final message
        let final_msg = ChatMessage { role: "assistant".to_string(), content: response.to_string() };
        let final_json = format!("{}\n", serde_json::to_string(&final_msg).unwrap());
        stream.write(final_json.as_bytes()).unwrap();
        stream.flush().unwrap();


        // Get reply
        //let user_reply = UserReply::new(auth_user, &message.as_str());
    //let reply = user_reply.process();

    let res = api::response(200, "", "Yes, made it here".to_string());
    let json = format!("{}\n", res.body());
    stream.write(&json.as_bytes()).unwrap();
    stream.flush().unwrap();

        res
    }
}


