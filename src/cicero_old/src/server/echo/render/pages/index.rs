
use crate::server::echo::render::{AccountPage, ASSETS};

/// index page
pub fn render() -> AccountPage {

    let html = ASSETS.get("theme/index");

    let mut page = AccountPage::new()
        .title("Welcome to Cicero")
        .header()
        .html(&html)
        .footer();

    page
}


