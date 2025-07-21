
use std::collections::HashMap;
use serde_derive::{Serialize, Deserialize};
use url::Url;
use parsex::{Token, Stack};
use std::time::Instant;
use crate::error::Error;
use super::{Scraper, Page, PageLayout, Codex, LayoutComponent, page_post_process};
use log::{info, error, debug, warn};

pub struct Site {
    scraper: Scraper,
    pub url: Url,
    pub domain_name: String,
    pub start_time: Instant,
    pub queue: Vec<(String, u8)>,
    pub layout: PageLayout,
    pub codex: Codex,
    pub pages: Vec<String>,
    pub markdown: String
}

impl Site {

    /// Instantiate new site
    pub fn new(url_str: &str, scraper: &Scraper) -> Result<Self, Error> {

        // Scrape initial page
        let mut site_scraper = scraper.clone();
        info!("Starting scrape of site, {}", url_str);
        let mut page = Page::get(&url_str, &0, &mut site_scraper.http, &false, None)?;

        // Create layout
        let layout = match PageLayout::parse(&mut page) {
            Ok(r) => r,
            Err( Error::IndeterminateLayout(comp) ) => {
                error!("Unable to determine layout for site, skipping.");
                scraper.process_invalid_layout(&page, &comp);
                std::process::exit(0);
            },
            Err(e) => return Err(Error::Custom(e.to_string()))
        };
        debug!("Found and extracted site layoug.");

        // Extract layout from page
        if let Err(e) = layout.extract_body(&mut page.stack) {
            scraper.process_invalid_layout(&page, &LayoutComponent::EXTRACT_BODY);
            return Err (Error::Custom("Unable to extract body out of site layout, aborting.".to_string()));
        }

        // Instantiate site
        let site = Site {
            scraper: scraper.clone(),
            url: page.url.clone(),
            domain_name: page.url.host_str().unwrap().to_lowercase().trim_start_matches("www.").to_string(),
            start_time: Instant::now(),
            queue: page.codex.links.internal.clone().into_iter().map(|path| (path, 0)).collect(),
            layout,
            codex: page.codex.to_owned(),
            pages: vec![ page.url.path().to_string() ],
            markdown: format!("# {}\n\n{}\n\n", page.codex.title, page.markdown.code)
        };

        // Create new page codex
        page.codex = Codex::parse(&page.url, &page.stack, Some(&site.layout));
        Ok(site)
    }

    /// Check if link valid
    fn is_valid_link(&self, path: &str, depth: &u8) -> bool {

        // Check if already scraped
        if self.pages.contains(&path.to_string() ) {
            return false;
        } else if self.scraper.config.max_depth > 0 && *depth > self.scraper.config.max_depth {
            return false;
        }

        // Check exclude dirs
        for dir in self.scraper.config.excludes.iter() {
            if path.starts_with(dir) {
                return false;
            }
        }

        true
    }

    /// Finish scraping
    fn finish(&mut self) {
        info!("Finished scraping {} in {} seconds", self.domain_name, self.start_time.elapsed().as_secs());
    }

}

impl Iterator for Site {
    type Item = Page;

    /// Scrape the next page of web site in queue
    fn next(&mut self) -> Option<Self::Item> {

        // GO through queue
        loop {

            if self.queue.len() == 0 {
                self.finish();
                return None;
            }
            let (path, depth) = self.queue.remove(0);

            // Check if path valid
            if !self.is_valid_link(&path, &depth) {
                continue;
            }
            let url_str = format!("{}://{}/{}", self.url.scheme(), self.domain_name, path);
            let start_time = Instant::now();

            // Scrape page
            let mut page = match Page::get(&url_str, &depth, &mut self.scraper.http, &self.scraper.config.skip_irrelevant_pages, Some(&self.layout)) {
                Ok(r) => r,
                Err(e) => {
                    warn!("Unable to scrape page, {} due to error: {}", url_str, e.to_string());
                    continue;
                }
            };

            // Check if layout extracted
            if !page.extracted_layout {
                self.scraper.process_invalid_layout(&page, &LayoutComponent::EXTRACT_BODY);
                warn!("Unable to extract body from page layout of /{}", path);
            }

            // Add page to site's codex
            //self.codex.add_page(&mut page.stack);

            // Post process
        page_post_process::run(&self.scraper, &page);

            debug!("Scraped /{} in {}ms", path, start_time.elapsed().as_millis());
            return Some(page);
        }

        self.finish();
        None
    }

}

