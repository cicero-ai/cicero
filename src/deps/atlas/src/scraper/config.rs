
use std::collections::HashMap;
use atlas_http::{HttpSyncClient};

#[derive(Debug, Clone)]
pub struct ScraperConfig {
    pub http: Option<HttpSyncClient>,
    pub max_depth: u8,
    pub excludes: Vec<String>,
    pub skip_irrelevant_pages: bool,
    pub invalid_layout_save_dir: String,
    pub save_dir: String,
    pub save_html: bool,
    pub save_markdown: bool,
    pub save_codex: bool,
    pub save_format: FileFormat
}

#[derive(Debug, Clone)]
pub enum FileFormat {
    YAML,
    JSON
}


impl ScraperConfig {

    pub fn new() -> Self {

        Self {
            http: None,
            max_depth: 0,
            skip_irrelevant_pages: false,
            excludes: Vec::new(),
            invalid_layout_save_dir: String::new(),
            save_dir: String::new(),
            save_html: false,
            save_markdown: false,
            save_codex: false,
            save_format: FileFormat::YAML
        }
    }

    /// Set http client
    pub fn http_client(mut self, http: &HttpSyncClient) -> Self {
            self.http = Some(http.clone());
        self
    }

    /// Set max_depth
    pub fn max_depth(mut self, max_depth: u8) -> Self {
        self.max_depth = max_depth;
        self
    }

    /// Set excludes
    pub fn excludes(mut self, excludes: &Vec<&str>) -> Self {
        self.excludes = excludes.into_iter().map(|e| e.to_string()).collect();
        self
    }

    // Set skip_irrelevant_pages
    pub fn skip_irrelevant_pages(mut self, skip: bool) -> Self {
        self.skip_irrelevant_pages = skip;
        self
    }

    /// Set invalid_layout_save_dir
    pub fn invalid_layout_save_dir(mut self, save_dir: &str) -> Self {
        self.invalid_layout_save_dir = save_dir.to_string();
        self
    }

    // Set save directory
    pub fn save_dir(mut self, dir_name: &str) -> Self {
        self.save_dir = dir_name.trim_end_matches("/").to_string();
        self
    }

        // Set save_html
    pub fn save_html(mut self, save: bool) -> Self {
        self.save_html = save;
        self
    }

        // Set save_markdown
    pub fn save_markdown(mut self, save: bool) -> Self {
        self.save_markdown = save;
        self
    }

        // Set save codex
    pub fn save_codex(mut self, save: bool) -> Self {
        self.save_codex = save;
        self
    }

    /// Save format
    pub fn save_format(mut self, format: FileFormat) -> Self {
        self.save_format = format;
        self
    }

}


impl Default for ScraperConfig {
    fn default() -> ScraperConfig {
        Self::new()
    }
}


