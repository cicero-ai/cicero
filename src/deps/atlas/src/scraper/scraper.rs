
use atlas_http::{HttpClient, HttpSyncClient};
use url::Url;
use crate::error::Error;
use super::{ScraperConfig, Site, Page, PageLink, LinkType, LayoutComponent};
use std::path::Path;
use std::fs;
use log::{info, warn};

#[derive(Debug, Clone)]
pub struct Scraper {
    pub config: ScraperConfig,
    pub http: HttpSyncClient
}

impl Scraper {

    pub fn new(config: ScraperConfig) -> Self {

        // Start HTTP client
        let mut http = config.http.clone().unwrap_or_else(|| {
            HttpClient::builder().browser().build_sync()
        });

        Self {
            config,
            http
        }
    }

    /// Scrape single page
    pub fn scrape_page(&mut self, url_str: &str) -> Result<Page, Error> {
        let mut page = Page::get(&url_str, &0, &mut self.http, &false, None)?;
        Ok(page)
    }

    /// Start scraping web site
    pub fn scrape_site(&mut self, url_str: &str) -> Result<Site, Error> {
        let mut site = Site::new(&url_str, &self)?;
        Ok(site)
    }

    /// Process invalid page layout
    pub fn process_invalid_layout(&self, page: &Page, component: &LayoutComponent) {

        // Check if saving
        if self.config.invalid_layout_save_dir.is_empty() {
            return;
        }

        // Create directory, nf needed
        let alias = serde_json::to_string(&component).unwrap().to_lowercase().trim_end_matches('"').trim_start_matches('"').to_string();
        let dirname = format!("{}/{}", self.config.invalid_layout_save_dir, alias);
        if !Path::new(&dirname).exists() {
            fs::create_dir_all(&dirname).expect("Unable to create invalid layouts directory");
        }
        // Save file
        let filename = format!("{}/{}.html", dirname, page.url.host_str().unwrap().trim_start_matches("www.").to_string());
        fs::write(&filename, page.contents.clone())
            .expect("Unable to save page contents to invalid layouts dir");
    }



}

