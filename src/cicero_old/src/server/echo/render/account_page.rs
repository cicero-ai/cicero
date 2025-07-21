
use super::ASSETS;
use super::nav_menu;

pub struct AccountPage {
    title: String,
    components: Vec<(String, String)>
}

impl AccountPage {

    pub fn new() -> Self {
        Self {
            title: String::new(),
            components: Vec::new()
        }
    }

        /// Page header
    pub fn header(mut self) -> Self {
        self.components.push(("header".to_string(), String::new()));
        self
    }

    /// Page footer
    pub fn footer(mut self) -> Self {
        self.components.push(("footer".to_string(), String::new()));
        self
    }

    /// Set page title
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    /// Raw html
    pub fn html(mut self, html: &str) -> Self {
        self.components.push(("html".to_string(), html.to_string()));
        self
    }

    /// Render page
    pub fn render(&self) -> String {

        let mut html = String::new();
        for (tag, details) in self.components.iter() {

            let content = match tag.as_str() {
                "header" => ASSETS.get("theme/header"),
                "footer" => ASSETS.get("theme/footer"),
                "html" => details.to_string(),
                _ => "404 Not Found".to_string()
            };
            html.push_str(&content);
        }

        // Replace title
        html = html.replace("<cicero_page_title>", &self.title);
        html = html.replace("<cicero_nav_menu>", &nav_menu::render());

        html.to_string()
    }

}


