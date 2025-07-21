
use serde_derive::{Serialize, Deserialize};
use serde::Serializer;
use std::collections::HashMap;
use std::fs;
use crate::error::Error;
use super::{Scraper, SocialNetwork};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteSummary {
    pub domain_name: String,
    pub title: String,
    pub pages: HashMap<String, String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub external_links: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ftp_links: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub other_links: Vec<String>,
    pub social_profiles: HashMap<SocialNetwork, Vec<String>>,
    pub email: Vec<String>,
    pub phone: Vec<String>
}

impl SiteSummary {

    pub fn new(scraper: &mut Scraper) -> Result<Self, Error> {

        // Start summary
        let mut summary = Self {
            domain_name: scraper.site.domain_name.clone(),
            title: scraper.site.title.clone(),
            pages: scraper.site.pages.clone(),
            external_links: scraper.site.external_links.clone(),
            ftp_links: scraper.site.ftp_links.clone(),
            other_links: scraper.site.other_links.clone(),
            social_profiles: scraper.site.social_profiles.clone(),
            email: scraper.site.email.clone(),
            phone: scraper.site.phone.clone()
        };

        // Process summary
        summary.process(scraper);

        Ok(summary)
    }

    /// Process summary
    pub fn process(&mut self, scraper: &mut Scraper) -> Result<(), Error> {

        // Save size.yml file
        let site_contents = self.save_site_yml(scraper)?;

        // Save summary
        self.save_summary(scraper);

        Ok(())
    }

    /// Save siy.tml file
    fn save_site_yml(&mut self, scraper: &mut Scraper) -> Result<String, Error> {

        // Create markdonw
        let contents = scraper.site.page_markdown.iter().map(|(path, body)| {
            format!("----- {}\n{}\n", path, body)
        }).collect::<Vec<String>>().join("\n").to_string();

        // Save file
        if !scraper.config.save_dir.is_empty() {
            let filename = format!("{}/full_site.yml", scraper.config.save_dir.clone());
            match fs::write(filename.clone(), contents.clone()) {
                Ok(_) => { },
                Err(e) => return Err( Error::Scraper(format!("Unable to write to file {} due to error: {}", filename, e.to_string())))
            };
        }

        Ok(contents)
    }

    /// Save summary
    fn save_summary(&mut self, scraper: &mut Scraper) -> Result<(), Error> {

        // Initialize
        if scraper.config.save_dir.is_empty() {
            return Ok(());
        }
        let filename = format!("{}/summary.yml", scraper.config.save_dir);

        // Get summary
        let code = serde_yaml::to_string(&self).unwrap();
        match fs::write(filename.clone(), code) {
            Ok(_) => { },
            Err(e) => return Err( Error::Scraper(format!("Unable to write to file {} due to error: {}", filename, e.to_string())))
        };

        Ok(())
    }

}


