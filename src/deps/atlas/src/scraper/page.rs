
use atlas_http::{HttpSyncClient, HttpResponse};
use crate::error::Error;
use serde_derive::{Serialize, Deserialize};
use parsex::{Stack, Token};
use url::Url;
use regex::RegexBuilder;
use crate::scraper::markdown::Markdown;
use super::{PageLayout, Codex, LayoutComponent};

#[derive(Debug, Clone)]
pub struct Page {
    pub url: Url,
    pub url_str: String,
    pub depth: u8,
    pub extracted_layout: bool,
    pub contents: String,
    pub markdown: Markdown,
    pub stack: Stack,
    pub codex: Codex
}

impl Page {

    /// Download and parse a page
    pub fn get(
        url_str: &str, 
        depth: &u8, 
        http: &mut HttpSyncClient, 
        skip_irrelevant: &bool,
        page_layout: Option<&PageLayout>
    ) -> Result<Self, Error> {

        // Get uri
        let url = match Url::parse(&url_str) {
            Ok(r) => r,
            Err(e) => { return Err( Error::InvalidUri(url_str.to_string()) ); }
        };

        // Send http request
        let res = match http.get(&url_str) {
            Ok(r) => r,
            Err(e) => { return Err( Error::Scraper( e.to_string() ) ); }
        };

        // Skip if not relevant
        if *skip_irrelevant && !Self::is_relevant(&res) {
            return Err( Error::IrrelevantPage("Irrelevant page skiped.".to_string()));
        }

        // Instantiate page
        let mut page = Self {
            url: url.clone(),
            url_str: url_str.to_string(),
            depth: depth.clone(),
            extracted_layout: false,
            contents: res.body(),
            markdown: Markdown::default(),
            stack: parsex::parse_html(&res.body()),
            codex: Codex::default()
        };

        // Generate codex, and get stack for markdown generation
        let (mut is_site, mut markdown_stack) = (false, Stack::default());
        if let Some(layout) = page_layout {
            markdown_stack = layout.extract_body(&mut page.stack).unwrap();
            page.codex = Codex::parse(&url, &page.stack, page_layout);
            is_site = true;
        } else {
            page.codex = Codex::parse(&url, &page.stack, None);

            if let Ok(layout) = PageLayout::parse(&mut page) {
                if let Ok(tmp_stack) = layout.extract_body(&mut page.stack) {
                    markdown_stack = tmp_stack;
                    page.extracted_layout = true;
                }
            }
        }

        // Generate markdown
        page.markdown = Markdown::generate(&page.codex.title.as_str(), &mut markdown_stack);
        if is_site {
            page.stack = markdown_stack.to_owned();
        }

        Ok(page)
    }

    /// Check if relevent
    pub fn is_relevant(res: &HttpResponse) -> bool {
        return true;

        // Get title
        //let excludes = crate::ai::text::dictionary::excludes();
        let excludes: Vec<String> = Vec::new();
        let re = RegexBuilder::new(r"<title>(.*?)</title>").dot_matches_new_line(true).build().unwrap();
        if let Some(caps) = re.captures(&res.body()) {
            let chk_title = caps.get(1).unwrap().as_str().parse::<String>().unwrap().trim().to_lowercase().to_string();
            for ex in excludes {
                if chk_title.contains(&ex.as_str()) { return false; }
            }
        }

        true

    }
}
