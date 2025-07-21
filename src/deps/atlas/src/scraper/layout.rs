
use crate::error::Error;
use parsex::{Stack, Token};
use url::Url;
use serde_derive::{Serialize, Deserialize};
use super::{Page, PageLink, Codex};
use log::debug;

#[derive(Debug, Clone)]
pub struct PageLayout {
    pub body_tag: TokenIdentifier,
    pub nav_tag: Option<TokenIdentifier>,
    pub footer_tag: Option<TokenIdentifier>,
    pub excludes: Vec<TokenIdentifier>,
    pub nav: Vec<NavMenu>,
    pub header_codex: Option<Codex>,
    pub nav_codex: Option<Codex>,
    pub footer_codex: Option<Codex>
}

#[derive(Debug, Clone)]
pub struct NavMenu {
    menu: PageLink,
    pub children: Vec<NavMenu>
}

#[derive(Debug, Clone)]
pub struct TokenIdentifier {
    pub tag: String,
    pub class: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LayoutComponent {
    BODY,
    EXTRACT_BODY,
    NAV,
    FOOTER
}

impl PageLayout {

    /// Extract page layout from body
    pub fn extract_body(&self, stack: &Stack) -> Result<Stack, Error> {

        // Initialize
        let mut tmp_stack = stack.clone();
        let exclude_ids = self.get_exclude_token_ids(&mut tmp_stack);

        // Generate query
        let mut query = tmp_stack.query().tag(&self.body_tag.tag);
        if !self.body_tag.class.is_empty() {
            query = query.class(&self.body_tag.class);
        }

        // Check for body tag
        if let Some(tag) = query.iter().next() {
            return Ok(tmp_stack.clone_from(&tag.id(), &exclude_ids).unwrap());
        }

        Err(Error::Custom("Unable to extract bod from layout".to_string()))
    }

    // Parse layout from newly scraped site
    pub fn parse(page: &mut Page) -> Result<Self, Error> {

        // Initialize
        let mut layout = Self::default();
        let url = page.url.clone();

        // Get body tag
        let body_tag = layout.get_body_tag(&url, &mut page.stack)?;
        debug!("Found body tag of layout, tag: {}, class: {}", layout.body_tag.tag, layout.body_tag.class);

        // Get nav menu
        layout.get_nav_tag(&url, &mut page.stack)?;

        // Get footer
        layout.get_footer_tag(&url, &mut page.stack)?;

        // Update page stack
        let exclude_token_ids = layout.get_exclude_token_ids(&mut page.stack);
        //page.stack = page.stack.clone_from(&body_tag.id(), &exclude_token_ids).unwrap();

        Ok(layout)
    }

    /// Get body tag
    fn get_body_tag(&mut self, url: &Url, stack: &mut Stack) -> Result<Token, Error> {

        // Define body tags to search for
        let search_list = vec![
            ("div", "body"),
            ("div", "main"),
            ("div", "page-body-content"),
            ("div", "page-body"),
            ("div", "page-content"),
            ("div", "main-content"),
            ("div", "container"),
            ("div", "wrapper"),
            ("main", ""),
            ("section", "main"),
            ("section", "body"),
            ("div", "content-body"),
            ("section", "page-content"),
            ("article", "body"),
            ("div", "blog-post"),
            ("section", "blog-post"),
            ("article", "main-content"),
            ("article", "blog-post"),
            ("body", "")
        ];

        /// Run search
        let (ident, tag) = match self.run_search(&search_list, stack) {
            Some(r) => r,
            None => return Err(Error::IndeterminateLayout(LayoutComponent::BODY))
        };

        self.body_tag = ident;
        Ok(tag)
    }

    /// Get nav menu
    fn get_nav_tag(&mut self, url: &Url, stack: &mut Stack) -> Result<Token, Error> {

        // Define search terms
        let search_list = vec![
            ("nav", ""),
            ("ul", "nav"),
            ("ul", "navigation"),
            ("ul", "main-nav"),
            ("ul", "mainnav"),
            ("ul", "horizontal-nav"),
            ("div", "nav"),
            ("div", "navigation"),
            ("div", "main-nav"),
            ("header", "nav"),
            ("header", "navigation")
        ];

        /// Run search
        let (ident, tag) = match self.run_search(&search_list, stack) {
            Some(r) => r,
            None => return Err(Error::IndeterminateLayout(LayoutComponent::NAV))
        };

        let mut tmp_stack = stack.clone_from(&tag.id(), &vec![]).unwrap();
        self.nav_codex = Some(Codex::parse(&url, &mut tmp_stack, None));
        self.nav = self.parse_navmenu(&tag, stack.clone(), &url);

        self.excludes.push(ident.clone());
        self.nav_tag = Some(ident);
        Ok(tag)
    }

    /// Get footer tag
    fn get_footer_tag(&mut self, url: &Url, stack: &mut Stack) -> Result<Token, Error> {

            // Define search terms
        let search_list = vec![
            ("footer", ""),
            ("div", "footer"),
            ("section", "footer"),
            ("section", "footer-section"),
            ("div", "content-footer")
        ];

        /// Run search
        let (ident, tag) = match self.run_search(&search_list, stack) {
            Some(r) => r,
            None => return Err(Error::IndeterminateLayout(LayoutComponent::FOOTER))
        };

        let mut tmp_stack = stack.clone_from(&tag.id(), &vec![]).unwrap();
        self.footer_codex = Some(Codex::parse(&url, &mut tmp_stack, None));

        self.excludes.push(ident.clone());
        self.footer_tag = Some(ident);
        Ok(tag)
    }

    // Run the search
    fn run_search(&mut self, search_list: &Vec<(&str, &str)>, stack: &mut Stack) -> Option<(TokenIdentifier, Token)> {

        // Search for body tag
        for (tag_name, class_name) in search_list {
            let mut query = stack.query().tag(&tag_name);
            if !class_name.is_empty() {
                query = query.class(&class_name);
            }

            // Check for tag
            if let Some(tag) = query.iter().next() {
                let ident = TokenIdentifier::new(&tag_name, &class_name);
                return Some((ident, tag));
            }
        }

        None
    }

    /// Parse navigation menu
    fn parse_navmenu(&mut self, parent: &Token, mut stack: Stack, parent_uri: &Url) -> Vec<NavMenu> {

        // Go through menus
        let mut menus = Vec::new();
        for tag in parent.children(&mut stack).tag("li").iter() {

            if tag.depth() > parent.depth() {
                continue;
            }

            let anchor = match tag.children(&mut stack).tag("a").iter().next() {
                Some(r) => r,
                None => continue
            };

            let link = match PageLink::from_token(&anchor, &parent_uri) {
                Some(r) => r,
                None => continue
            };

            let mut children = Vec::new();
            if let Some(child_parent) = tag.children(&mut stack).tag("ul").iter().next() {
                children = self.parse_navmenu(&child_parent, stack.clone(), &parent_uri);
            }

            // Add to nav menu
            menus.push( NavMenu {
                menu: link,
                children: children
            });
        }

        menus
    }

    /// Get exclude token ids
    fn get_exclude_token_ids(&self, stack: &mut Stack) -> Vec<usize> {

        let mut token_ids = Vec::new();
        for ident in self.excludes.clone() {
            let mut query = stack.query().tag(&ident.tag);
            if !ident.class.is_empty() {
                query = query.class(&ident.class);
            }

            if let Some(tag) = query.iter().next() {
                token_ids.push(tag.id().clone());
            }
        }

        token_ids
    }

}

impl Default for PageLayout {
    fn default() -> PageLayout {
        PageLayout {
            body_tag: TokenIdentifier::default(),
            nav_tag: None,
            footer_tag: None,
            excludes: Vec::new(),
            nav: Vec::new(),
            header_codex: None,
            nav_codex: None,
            footer_codex: None
        }
    }

}

impl TokenIdentifier {
    pub fn new(tag_name: &str, class_name: &str) -> Self {
        Self {
            tag: tag_name.to_string(),
            class: class_name.to_string()
        }
    }
}

impl Default for TokenIdentifier {
    fn default() -> TokenIdentifier {
        TokenIdentifier {
            tag: String::new(),
            class: String::new()
        }
    }

}


