

use atlas_http::HttpBody;
use log::error;
use crate::utils::api_client;
use crate::server::echo::render::{AccountPage, ASSETS};
use crate::CLIENT_CONFIG;

pub fn render() -> AccountPage {

    // Check in
    let prompt = match api_client::send_body::<String>("v1/chat/checkin", "GET", &HttpBody::empty()) {
        Ok(r) => r,
        Err(e) => {
            error!("Did not receive valid response from Apollo server, unable to check in with chat conversation.");
            String::new()
        }
    };

    // Get nickname
    let mut nickname = "Guest".to_string();
    if let Some(user) = &CLIENT_CONFIG.current_user {
        nickname = user.nickname.to_string();
    }

    // Get html code
    let mut html = ASSETS.get("theme/chat");
    html = html.replace("~prompt~", &prompt);
    html = html.replace("~nickname~", &nickname);

    // Render the page
    let mut page = AccountPage::new()
        .title("Chat with Cicero")
        .header()
        .html(&html)
        .footer();

    page
}



