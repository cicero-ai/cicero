
use regex::Regex;
pub use self::process_manager::ProcessManager;
pub use self::scrape_wikipedia::WikipediaScraper;

pub mod api_client;
pub mod process_manager;
pub mod random;
pub mod scrape_wikipedia;
pub mod sys;

/// Check if string is alpha-numeric
pub fn is_alpha_numeric(text: &str) -> bool {
    let regex = r"^[a-zA-Z0-9_-]+$";
    Regex::new(regex).unwrap().is_match(text)
}



