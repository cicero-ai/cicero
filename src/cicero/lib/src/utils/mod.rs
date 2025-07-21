
use regex::Regex;
pub use self::process_manager::ProcessManager;


mod process_manager;
pub mod random;
pub mod sys;

/// Check if string is alpha-numeric
pub fn is_alpha_numeric(text: &str) -> bool {
    let regex = r"^[a-zA-Z0-9_-]+$";
    Regex::new(regex).unwrap().is_match(text)
}



