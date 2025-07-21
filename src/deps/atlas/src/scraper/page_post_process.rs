
use std::path::Path;
use std::fs;
use crate::error::Error;
use super::{Scraper, Page};

/// Process page
pub fn run(scraper: &Scraper, page: &Page) -> Result<(), Error> {

    // Get save dir
    let save_dir = scraper.config.save_dir.replace("~host~", page.url.host_str().unwrap().trim_start_matches("www.").to_string().as_str());


    // Save html
    if scraper.config.save_html && !scraper.config.save_dir.is_empty() {
        save_html(&save_dir, page)?;
    }

    // Save markdown
    if scraper.config.save_markdown && !scraper.config.save_dir.is_empty() {
        save_markdown(&save_dir, page)?;
    }

    // Save codex
    if scraper.config.save_codex && !scraper.config.save_dir.is_empty() {
        save_codex(&save_dir, page)?;
    }

    Ok(())
}

/// Save .html file
fn save_html(dirname: &String, page: &Page) -> Result<(), Error> {

    // Initialize
    let short_name = if page.url.path() == "/" || page.url.path() == "" { "index" } else { page.url.path().trim_end_matches("/") };
    let filename = format!("{}/html/{}.html", dirname, short_name);
    let parent_dir = Path::new(&filename).parent().unwrap();

    // Create parent dir, if needed
    if !Path::new(&parent_dir).exists() {
        match fs::create_dir_all(&parent_dir) {
            Ok(_) => { },
            Err(e) => return Err( Error::Scraper(format!("Unable to create parent directory for file {} due to error: {}", filename, e.to_string())))
        };
    }

    // Save file
    match fs::write(filename.clone(), page.contents.clone()) {
        Ok(_) => { },
        Err(e) => return Err( Error::Scraper(format!("Unable to write to file {} due to error: {}", filename, e.to_string())))
    };

    Ok(())
}


/// Save markdown file
fn save_markdown(dirname: &String, page: &Page) -> Result<(), Error> {

    // Initialize
    let short_name = if page.url.path() == "/" || page.url.path() == "" { "index" } else { page.url.path().trim_end_matches("/") };
    let filename = format!("{}/md/{}.md", dirname, short_name);
    let parent_dir = Path::new(&filename).parent().unwrap();

    // Create parent dir, if needed
    if !Path::new(&parent_dir).exists() {
        match fs::create_dir_all(&parent_dir) {
            Ok(_) => { },
            Err(e) => return Err( Error::Scraper(format!("Unable to create parent directory for file {} due to error: {}", filename, e.to_string())))
        };
    }

    // Save file
    match fs::write(filename.clone(), page.markdown.code.clone()) {
        Ok(_) => { },
        Err(e) => return Err( Error::Scraper(format!("Unable to write to file {} due to error: {}", filename, e.to_string())))
    };

    Ok(())
}


/// Save codex
fn save_codex(dirname: &str, page: &Page) -> Result<(), Error> {

    // Initialize
    let short_name = if page.url.path() == "/" || page.url.path() == "" { "index" } else { page.url.path().trim_end_matches("/") };
    let filename = format!("{}/summary/{}.yml", dirname, short_name);
    let parent_dir = Path::new(&filename).parent().unwrap();

    // Create parent dir, if needed
    if !Path::new(&parent_dir).exists() {
        match fs::create_dir_all(&parent_dir) {
            Ok(_) => { },
            Err(e) => return Err( Error::Scraper(format!("Unable to create parent directory for file {} due to error: {}", filename, e.to_string())))
        };
    }

    // Get codex
    let code = serde_yaml::to_string(&page.codex).unwrap();

    // Save file
    match fs::write(filename.clone(), code) {
        Ok(_) => { },
        Err(e) => return Err( Error::Scraper(format!("Unable to write to file {} due to error: {}", filename, e.to_string())))
    };

    Ok(())
}

