
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HtmlTable {
    pub columns: Vec<String>,
}

impl HtmlTable {

}


impl Default for HtmlTable {
    fn default() -> HtmlTable {
        HtmlTable {
            columns: Vec::new()
        }
    }
}


