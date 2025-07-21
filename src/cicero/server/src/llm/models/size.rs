
#[derive(Eq, PartialEq, Clone)]
pub enum ModelSize {
    tiny,
    small,
    medium,
    large
}

impl ModelSize {
    pub fn to_string(&self) -> String {
        match self {
            Self::tiny => "tiny".to_string(),
            Self::small => "small".to_string(),
            Self::medium => "medium".to_string(),
            Self::large => "large".to_string()
        }
    }
}




