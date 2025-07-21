
use std::collections::HashMap;
use crate::llm::text::{PreProcessor, PreProcessorConfig};
use serde_derive::{Serialize, Deserialize};
use std::fs;
use atlas_http::HttpClientBuilder;
use parsex::Stack;
use crate::server::CONFIG;
use log::info;

static SEED_URLS: &'static [&str] = &[
    "/wiki/Business"
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikipediaScraper {
    queue: Vec<String>,
    queue_done: Vec<String>,
    pre_processor: PreProcessor,
    verbs: Vec<String>,
    nouns: Vec<String>,
    words: HashMap<String, HashMap<String, usize>>
}

impl WikipediaScraper {

    pub fn new() -> Self {
        let mut config = PreProcessorConfig::builder().do_stem(true).remove_punctuation(true);
        Self { 
            queue: SEED_URLS.into_iter().map(|url| url.to_string()).collect(),
            queue_done: Vec::new(),
            pre_processor: PreProcessor::new(config),
            verbs: fs::read_to_string(format!("{}/nlu/en/vocab/verbs", CONFIG.general.libdir)).unwrap().split("\n").map(|word| word.to_string()).collect::<Vec<String>>(),
            nouns: fs::read_to_string(format!("{}/nlu/en/vocab/nouns", CONFIG.general.libdir)).unwrap().split("\n").map(|word| word.to_string()).collect::<Vec<String>>(),
            words: HashMap::new()
        }
    }

    pub fn scrape(&mut self) {

        // Initialize
        let mut http = HttpClientBuilder::new().browser().build_sync();
        let mut total = 0;

        // Go through urls
        loop {

            if self.queue.len() == 0 {
                break;
            }
            let path = self.queue.remove(0);

            // Scrape the page
            let url = format!("https://en.wikipedia.org{}", path);
            let res = http.get(&url).unwrap();
            let mut stack = parsex::parse_html(&res.body());

            // Go through urls, process queue
            self.add_queue(&mut stack);

            // Add paragraphs to TF-IDF
            for tag in stack.query().tag("p").iter() {
                self.add_terms(&tag.strip_tags().as_str());
            }

            info!("Scraped {}", path);
            total += 1;
            if total % 10 == 0 {
                println!("Got to 1000, quitting");
                break;
            }
        if total > 10 { println!("Should not be here"); break; }
        }

    for (verb, map) in self.words.iter() {
        println!("Verb: {}", verb);
        for (noun, count) in map.iter() {
            println!("    Nount: {} -- {}", noun, count);
        }
    }




    }

    /// Add urls to queue
    fn add_queue(&mut self, stack: &mut Stack) {

        for tag in stack.query().tag("a").iter() {
            let href = match tag.attr("href") {
                Some(r) => r,
                None => continue
            };
            if href.starts_with("/wiki/Special:") || !href.starts_with("/wiki/") { 
                continue;
            } else if self.queue.contains(&href.to_string()) || self.queue_done.contains(&href.to_string()) {
                continue;
            }
            self.queue.push(href.to_string());
        }

    }

    /// Add terms to hashmap
    fn add_terms(&mut self, input: &str) {

        // Go through words

        // Pre-processor
        let clean_input = self.pre_processor.process(&input);
        let words: Vec<String> = clean_input[0].split(" ").map(|w| w.to_string()).collect();
        let mut verb = String::new();
        let mut nouns = Vec::new();

        // Go through all words
        for word in words {

            if self.nouns.contains(&word) {
                nouns.push(word.to_string());
                continue;
            } else if !self.verbs.contains(&word) {
                continue;
            }

            // Add nouns to previous verb, if needed
            if !verb.is_empty() {
                let mut word_map = self.words.entry(verb.clone()).or_default();
                for noun in nouns.iter() {
                    *word_map.entry(noun.to_string()).or_insert(0) += 1;
                }
            }

            // Add nouns to new verb
            verb = word.to_string();
            let mut word_map = self.words.entry(verb.clone()).or_default();
            for noun in nouns.iter() {
                *word_map.entry(noun.to_string()).or_insert(0) += 1;
            }
            nouns = Vec::new();
        }

    }

}

