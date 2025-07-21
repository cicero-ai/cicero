
pub use self::google::Google;

pub mod google;

pub struct SearchResult {
    pub url: String,
    pub title: String,
    pub description: Option<String>
}

