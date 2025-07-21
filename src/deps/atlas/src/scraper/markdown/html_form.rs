
use serde_derive::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HtmlForm {
    pub action: String,
    pub method: String,
    pub enctype: String,
    pub attr: HashMap<String, String>,
    fields: Vec<FormField>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormField  {
    field_type: String,
    name: String,
    default_value: String
}


