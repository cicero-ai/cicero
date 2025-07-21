
use atlas_http::{HttpClient};
use urlencoding::{decode, encode};
use parsex;
use super::SearchResult;

pub struct Google { 
    pub tld: String
}

impl Google {

    pub fn new(tld: &str) -> Self {
        Self { 
            tld: tld.to_string()
        }
    }

    /// Search for person
    pub fn search(&self, query: &str, num_results: u16) -> Vec<SearchResult> {

        // Initialize
        let mut http = HttpClient::builder().browser().build_sync();
        let url = format!("https://www.google.{}/search?q={}&gws_rd=ssl&num={}", self.tld, encode(query), num_results);

        // Send request
        let res = http.get(&url).unwrap();
        let mut stack = parsex::parse_html(&res.body());

        // Go through results
        let mut results: Vec<SearchResult> = Vec::new();
        for tag in stack.query().tag("div").class("g").iter() {

            // Get title
            let h3 = match stack.get_children(&tag.id()).tag("h3").iter().next() {
                Some(r) => r,
                None => { println!("No h3"); continue; }
            };

            // Get anchor tag
            let a = match stack.get_children(&tag.id()).tag("a").iter().next() {
                Some(r) => r,
                None => { println!("No anchor"); continue; }
            };

            // Get url
            let url = match a.attr("href") {
                Some(r) => r,
                none => continue
            };

            // Add to results
            results.push( SearchResult {
                url,
                title: h3.contents(),
                description: None
            });
        }

        results
    }

    /// Get country extension
    pub fn get_tld(country: &String) -> String {

        let tld = match country.to_lowercase().as_str() {
            "au" => "com.au",
            "ca" => "ca",
            "uk" => "co.uk",
            _ => "com"
        };

        tld.to_string()
    }

}


