
use std::collections::{HashSet, HashMap};
use serde_derive::{Serialize, Deserialize};
use url::Url;
use phonenumber::PhoneNumber;
use parsex::Stack;
use crate::Location;
use super::{LinkType, SocialNetwork, PageLayout, PageLink};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Codex {
    pub base_url: String,
    pub title: String,
    pub logo: Option<SiteLogo>,
    pub meta_tags: HashMap<String, String>,
    pub headings: HashMap<u8, String>,
    pub links: CodexLinks,
    pub emails: HashSet<String>,
    pub phone_numbers: HashSet<PhoneNumber>,
    pub social_profiles: Vec<(SocialNetwork, String)>,
    pub locations: HashSet<Location>,
    pub news_headlines: Vec<NewsHeadline>,
    pub relations: Vec<Relation>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteLogo {
    pub width: u16,
    pub height: u16,
    pub mime_type: String,
    pub contents: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodexLinks {
    pub domain_names: HashSet<String>,
    pub internal: HashSet<String>,
    pub news: HashSet<String>,
    pub directory: HashSet<String>,
    pub ecommerce: HashSet<String>,
    pub download: HashSet<String>,
    pub video: HashSet<String>,
    pub image: HashSet<String>,
    pub social: HashSet<String>,
    pub other: HashSet<String>,
    pub non_http: HashSet<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsHeadline {
    pub url: String,
    pub domain_name: String,
    pub news_date: u32,
    pub title: String,
    pub description: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum RelationType {
    Founder,
    Employee,
    Associate,
    Customer,
    Partner,
    Commentor,
    Other
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    pub relationship: RelationType,
    pub full_name: String,
    pub company_name: String,
    pub role: Option<String>,
    pub department: Option<String>,
    pub url: Option<String>,
    pub email: Option<String>,
    pub phone: Option<PhoneNumber>,
    pub known_since: u32,
    description: String
}


impl Codex {

    pub fn parse(url: &Url, borrowed_stack: &Stack, page_layout: Option<&PageLayout>) -> Self {

        // Start codex
        let mut codex = Codex::default();
        codex.base_url = format!("{}://{}{}", url.scheme(), url.host_str().unwrap(), url.path());
        let mut stack = borrowed_stack.clone();

        // Collect title and meta tags
        codex.collect_meta_tags(&mut stack);

        // Extract layout, if necessary
        if let Some(layout) = page_layout {
            stack = layout.extract_body(&stack.clone()).unwrap().to_owned();
        }

        // Collect links
        codex.collect_links(&url, &mut stack);

        codex
    }

    /// Add new page to codex
    pub fn add_page(&mut self, stack: &mut Stack) {

    }

    /// Collect title and meta tags
    fn collect_meta_tags(&mut self, stack: &mut Stack) {

        // Get title
        if let Some(title_tag) = stack.query().tag("title").iter().next() {
            self.title = title_tag.strip_tags();
        }

        // Collect meta tags
    for tag in stack.query().tag("meta").iter() {
            if (!tag.has_attr("name")) || (!tag.has_attr("content")) {
                continue;
            }
            self.meta_tags.insert(
                tag.attr("name").unwrap().to_lowercase().to_string(),
                tag.attr("content").unwrap().to_lowercase().to_string()
            );
        }

    }

    /// Collect links
    fn collect_links(&mut self, url: &Url, stack: &mut Stack) {

        // Go through all tags, gather queue
        for tag in stack.query().tag("a").excludes(&vec![]).iter() {

            let link = match PageLink::from_token(&tag, &url) {
                Some(r) => r,
                None => continue
            };

            // Process link type
            match link.link_type {
                LinkType::INTERNAL => self.links.internal.insert(link.href),
                LinkType::NEWS => self.links.news.insert(link.href),
                LinkType::DIRECTORY => self.links.directory.insert(link.href),
                LinkType::ECOMMERCE => self.links.ecommerce.insert(link.href),
                LinkType::DOWNLOAD => self.links.download.insert(link.href),
                LinkType::VIDEO => self.links.video.insert(link.href),
                LinkType::IMAGE => self.links.image.insert(link.href),
                LinkType::SOCIAL => self.links.social.insert(link.href),
                LinkType::OTHER => self.links.other.insert(link.href),
                LinkType::NON_HTTP => self.links.non_http.insert(link.href),
                LinkType::SOCIAL_PROFILE => {
                    self.social_profiles.push((link.social_network, link.social_username));
                    true
                }
            };

            // Add social profile
            if link.link_type == LinkType::SOCIAL_PROFILE {
                //self.social_profiles.push((link.social_network, link.social_username));
            }

            // Add to domain name
            if link.link_type != LinkType::INTERNAL && link.link_type != LinkType::NON_HTTP {
                self.links.domain_names.insert(link.domain_name);
            }
        }
    }

}

impl Default for Codex {
    fn default() -> Codex {
        Codex {
            base_url: String::new(),
            title: String::new(),
            logo: None,
            meta_tags: HashMap::new(),
            headings: HashMap::new(),
            links: CodexLinks::default(),
            emails: HashSet::new(),
            phone_numbers: HashSet::new(),
            social_profiles: Vec::new(),
            locations: HashSet::new(),
            news_headlines: Vec::new(),
            relations: Vec::new()
        }
    }
}

impl Default for CodexLinks {
    fn default() -> CodexLinks {
        CodexLinks {
            domain_names: HashSet::new(),
            internal: HashSet::new(),
            news: HashSet::new(),
            directory: HashSet::new(),
            ecommerce: HashSet::new(),
            download: HashSet::new(),
            video: HashSet::new(),
            image: HashSet::new(),
            social: HashSet::new(),
            other: HashSet::new(),
            non_http: HashSet::new()
        }
    }
}


impl CodexLinks {

    pub fn external(&self) -> HashSet<String> {

        let links: HashSet<String> = self.news
            .union(&self.directory)
            .chain(&self.ecommerce)
            .chain(&self.download)
            .chain(&self.video)
            .chain(&self.image)
            .chain(&self.social)
            .chain(&self.other)
            .cloned().collect();

        links
    }

}


