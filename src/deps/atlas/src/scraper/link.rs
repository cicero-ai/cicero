
use url::Url;
use parsex::Token;
use serde_derive::{Serialize, Deserialize};
use crate::search::SearchResult;
use crate::lists::websites::{IMAGE_EXTENSIONS, DOWNLOADABLE_FILE_EXTENSIONS};
use crate::lists::domains::{NEWS_DOMAINS, SOCIAL_DOMAINS, VIDEO_DOMAINS, ECOMMERCE_DOMAINS, DIRECTORY_DOMAINS};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageLink {
    pub link_type: LinkType,
    pub name: String,
    pub href: String,
    pub domain_name: String,
    pub social_network: SocialNetwork,
    pub social_username: String
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LinkType {
    INTERNAL,
    NEWS,
    DIRECTORY,
    ECOMMERCE,
    DOWNLOAD,
    VIDEO,
    IMAGE,
    SOCIAL,
    SOCIAL_PROFILE,
    OTHER,
    NON_HTTP
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Copy, Serialize, Deserialize)]
pub enum SocialNetwork {
    None,
    Facebook,
    Twitter,
    Instagram,
    Ticktock,
    Whatsapp,
    Reddit,
    Pinterest,
    Youtube,
    Linkedin,
    Tumblr,
    Github
}

impl PageLink {

    /// Generate link from token
    pub fn from_token(tag: &Token, parent_url: &Url) -> Option<PageLink> { 

        if !tag.has_attr("href") {
            return None;
        }
        let href = tag.attr("href").unwrap();

        // Create link
        Self::new(&href.as_str(), &tag.strip_tags().as_str(), Some(parent_url.clone()))
    }

    /// From single url
    pub fn from_url(url: &str) -> Option<PageLink> {
        Self::new(&url, "", None)
    }

    /// From search result
    pub fn from_search_result(res: &SearchResult) -> Option<PageLink> {
        Self::new(&res.url, &res.title, None)
    }

    // Create new instance of page link
    pub fn new(href: &str, title: &str, parent_url: Option<Url>) -> Option<PageLink> {

        // Start link
        let mut link = PageLink {
            link_type: LinkType::INTERNAL,
            name: title.to_string(),
            href: href.to_string(),
            domain_name: String::new(),
            social_network: SocialNetwork::None,
            social_username: String::new()
        };

        // Parse link
        link.parse(parent_url);
        Some(link)
    }

    /// Parse link
    fn parse(&mut self, parent_url: Option<Url>) {

        // Check for internal link
        let url = match self.check_for_internal_link(parent_url.as_ref()) {
            Some(r) => r,
            None => return
        };
        let chk_host = url.host_str().unwrap().trim_start_matches("www.");
        self.domain_name = chk_host.to_string();

        // Get link type
        self.get_link_type(&url);

        // Check for social network
        if self.link_type == LinkType::SOCIAL {
            self.check_for_social_profile(&url);
        }

    }

    /// Get link type
    fn get_link_type(&mut self, url: &Url) {

        let chk_host = url.host_str().unwrap().to_lowercase().trim_start_matches("www.").to_string();
        if url.scheme() != "http" && url.scheme() != "https" {
            self.link_type = LinkType::NON_HTTP;
        } else if self.is_image() {
            self.link_type = LinkType::IMAGE;
        } else if self.is_download() {
            self.link_type = LinkType::DOWNLOAD;
        } else if NEWS_DOMAINS.contains(&chk_host.as_str()) {
            self.link_type = LinkType::NEWS;
        } else if SOCIAL_DOMAINS.contains(&chk_host.as_str()) {
            self.link_type = LinkType::SOCIAL;
        } else if VIDEO_DOMAINS.contains(&chk_host.as_str()) {
            self.link_type = LinkType::VIDEO;
        } else if DIRECTORY_DOMAINS.contains(&chk_host.as_str()) {
            self.link_type = LinkType::DIRECTORY;
        } else if ECOMMERCE_DOMAINS.contains(&chk_host.as_str()) {
            self.link_type = LinkType::ECOMMERCE;
        } else {
            self.link_type = LinkType::OTHER;
        }

    }

    /// Check for internal link
    fn check_for_internal_link(&mut self, parent_url: Option<&Url>) -> Option<Url> {

        // Get parent host
        let mut parent_host = "";
        if let Some(parent) = parent_url {
            parent_host = parent.host_str().unwrap().clone().trim_start_matches("www.");
        }

        // Parse href 
        let url = match Url::parse(&self.href) {
            Ok(r) => r,
            Err(e) => {
                self.format_internal_href(&parent_url);
                return None;
            }
        };
        let chk_host = url.host_str().unwrap().trim_start_matches("www.");

        // Check for internal link with full URL including domain
        if chk_host == parent_host.clone() {
            self.href = url.path().clone().to_string();
            return None;
        }

        Some(url.to_owned())
    }

    /// FOrmat path / href
    fn format_internal_href(&mut self, parent: &Option<&Url>) {

        // Check for starting slash
        if self.href.starts_with("/") || parent.is_none() {
            self.href = self.href.trim_start_matches("/").to_string();
            return;
        }
        let parent_url = parent.unwrap();

        // Format parent path
        let is_trailing_slash = if parent_url.path().ends_with("/") && parent_url.path() != "/" { true } else { false };
        let parent_path = parent_url.path().trim_start_matches("/").trim_end_matches("/").to_string();

        let mut path = format!("{}/{}", parent_path, self.href);
        if !is_trailing_slash {
            if let Some(index) = parent_path.rfind('/') {
                path = format!("{}/{}", parent_path[..index].to_string(), self.href);
            } else {
                path = self.href.clone();
            }
        }
        self.href = format!("/{}", path);
    }

    /// Check for social profile
    fn check_for_social_profile(&mut self, url: &Url) {

        // Initialize
        let uri_parts: Vec<String> = url.path().trim_start_matches("/").trim_end_matches("/").split("/").map(|e| e.to_string()).collect();
        let chk_host = url.host_str().unwrap().trim_start_matches("www.");

        // Get social network
        let social_network = match chk_host {
            "facebook.com" => SocialNetwork::Facebook,
            "twitter.com" | "x.com" => SocialNetwork::Twitter,
            "instagram.com" => SocialNetwork::Instagram,
            "ticktock.com" => SocialNetwork::Ticktock,
            "wa.com" => SocialNetwork::Whatsapp,
            "linkedin.com" => SocialNetwork::Linkedin,
            "reddit.com" => SocialNetwork::Reddit,
            "pinterest.com" => SocialNetwork::Pinterest,
            "youtube.com" => SocialNetwork::Youtube,
            "tumblr.com" => SocialNetwork::Tumblr,
            "github.com" => SocialNetwork::Tumblr,
            _ => SocialNetwork::None
        };

        if social_network != SocialNetwork::None && uri_parts.len() == 1 {
            self.link_type = LinkType::SOCIAL_PROFILE;
            self.social_network = social_network;
            self.social_username = uri_parts[0].clone();
        } else if (social_network == SocialNetwork::Reddit && uri_parts[0] == "user") || (social_network == SocialNetwork::Linkedin && uri_parts[0] == "in") {
            self.link_type = LinkType::SOCIAL_PROFILE;
            self.social_network = social_network;
            self.social_username = uri_parts[1].clone();
        }

    }

    /// Is image link?
    pub fn is_image(&self) -> bool {
        if let Some(index) = self.href.rfind('.') {
            let ext = self.href[index..].to_lowercase().to_string();
            return IMAGE_EXTENSIONS.contains(&ext.as_str());
        }
        false
    }

    /// Is downloadable file link?
    pub fn is_download(&self) -> bool {
        if let Some(index) = self.href.rfind('.') {
            let ext = self.href[index..].to_lowercase().to_string();
            return IMAGE_EXTENSIONS.contains(&ext.as_str());
        }
        false
    }

}



