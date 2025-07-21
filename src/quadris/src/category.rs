
use std::fmt;
use std::error::Error;
use std::num::ParseIntError;

#[derive(Debug)]
enum Category {
    arts_and_culture,
    community_involvement,
    career_development_and_advancement,
    daily_routines,
    educational_interests,
    entertainment_and_media,
    exercise_and_fitness,
    family_and_relationships,
    food_and_drink,
    health_and_wellness,
    hobbies_and_creativity,
    home_and_housing,
    personal_growth_and_development,
    personal_finance_and_budgeting,
    pets_and_animals,
    philanthropic_endeavors,
    political_beliefs,
    shopping_and_fashion,
    socializing,
    spirituality_and_faith,
    sports_and_games,
    technology_and_computing,
    travel_and_exploration,
    transportation_and_vehicles,
}

#[derive(Debug)]
enum CategoryError {
    InvalidU8Value(u8),
    InvalidStringValue(String),
    ParseIntError(ParseIntError),
}

impl Error for CategoryError {}

impl fmt::Display for CategoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CategoryError::InvalidU8Value(value) => write!(f, "Invalid u8 value: {}", value),
            CategoryError::InvalidStringValue(value) => write!(f, "Invalid string value: {}", value),
            CategoryError::ParseIntError(err) => write!(f, "Parse int error: {}", err),
        }
    }
}

impl Category {
    pub fn to_u8(&self) -> u8 {
        match self {
            Category::arts_and_culture => 1,
            Category::community_involvement => 2,
            Category::career_development_and_advancement => 3,
            Category::daily_routines => 4,
            Category::educational_interests => 5,
            Category::entertainment_and_media => 6,
            Category::exercise_and_fitness => 7,
            Category::family_and_relationships => 8,
            Category::food_and_drink => 9,
            Category::health_and_wellness => 10,
            Category::hobbies_and_creativity => 11,
            Category::home_and_housing => 12,
            Category::personal_growth_and_development => 13,
            Category::personal_finance_and_budgeting => 14,
            Category::pets_and_animals => 15,
            Category::philanthropic_endeavors => 16,
            Category::political_beliefs => 17,
            Category::shopping_and_fashion => 18,
            Category::socializing => 19,
            Category::spirituality_and_faith => 20,
            Category::sports_and_games => 21,
            Category::technology_and_computing => 22,
            Category::travel_and_exploration => 23,
            Category::transportation_and_vehicles => 24,
        }
    }

    pub fn from_u8(value: u8) -> Result<Self, CategoryError> {
        match value {
            1 => Ok(Category::arts_and_culture),
            2 => Ok(Category::community_involvement),
            3 => Ok(Category::career_development_and_advancement),
            4 => Ok(Category::daily_routines),
            5 => Ok(Category::educational_interests),
            6 => Ok(Category::entertainment_and_media),
            7 => Ok(Category::exercise_and_fitness),
            8 => Ok(Category::family_and_relationships),
            9 => Ok(Category::food_and_drink),
            10 => Ok(Category::health_and_wellness),
            11 => Ok(Category::hobbies_and_creativity),
            12 => Ok(Category::home_and_housing),
            13 => Ok(Category::personal_growth_and_development),
            14 => Ok(Category::personal_finance_and_budgeting),
            15 => Ok(Category::pets_and_animals),
            16 => Ok(Category::philanthropic_endeavors),
            17 => Ok(Category::political_beliefs),
            18 => Ok(Category::shopping_and_fashion),
            19 => Ok(Category::socializing),
            20 => Ok(Category::spirituality_and_faith),
            21 => Ok(Category::sports_and_games),
            22 => Ok(Category::technology_and_computing),
            23 => Ok(Category::travel_and_exploration),
            24 => Ok(Category::transportation_and_vehicles),
            _ => Err(CategoryError::InvalidU8Value(value)),
        }
    }

    pub fn to_str(&self) -> String {
        match self {
            Category::arts_and_culture => "arts and culture".to_string(),
            Category::community_involvement => "community involvement".to_string(),
            Category::career_development_and_advancement => "career development and advancement".to_string(),
            Category::daily_routines => "daily routines".to_string(),
            Category::educational_interests => "educational interests".to_string(),
            Category::entertainment_and_media => "entertainment and media".to_string(),
            Category::exercise_and_fitness => "exercise and fitness".to_string(),
            Category::family_and_relationships => "family and relationships".to_string(),
            Category::food_and_drink => "food and drink".to_string(),
            Category::health_and_wellness => "health and wellness".to_string(),
            Category::hobbies_and_creativity => "hobbies and creativity".to_string(),
            Category::home_and_housing => "home and housing".to_string(),
            Category::personal_growth_and_development => "personal growth and development".to_string(),
            Category::personal_finance_and_budgeting => "personal finance and budgeting".to_string(),
            Category::pets_and_animals => "pets and animals".to_string(),
            Category::philanthropic_endeavors => "philanthropic endeavors".to_string(),
            Category::political_beliefs => "political beliefs".to_string(),
            Category::shopping_and_fashion => "shopping and fashion".to_string(),
            Category::socializing => "socializing".to_string(),
            Category::spirituality_and_faith => "spirituality and faith".to_string(),
            Category::sports_and_games => "sports and games".to_string(),
            Category::technology_and_computing => "technology and computing".to_string(),
            Category::travel_and_exploration => "travel and exploration".to_string(),
            Category::transportation_and_vehicles => "transportation and vehicles".to_string(),
        }
    }

    pub fn from_str(value: &str) -> Result<Self, CategoryError> {
        match value.to_lowercase().as_str() {
            "arts and culture" => Ok(Category::arts_and_culture),
            "community involvement" => Ok(Category::community_involvement),
            "career development and advancement" => Ok(Category::career_development_and_advancement),
            "daily routines" => Ok(Category::daily_routines),
            "educational interests" => Ok(Category::educational_interests),
            "entertainment and media" => Ok(Category::entertainment_and_media),
            "exercise and fitness" => Ok(Category::exercise_and_fitness),
            "family and relationships" => Ok(Category::family_and_relationships),
            "food and drink" => Ok(Category::food_and_drink),
            "health and wellness" => Ok(Category::health_and_wellness),
            "hobbies and creativity" => Ok(Category::hobbies_and_creativity),
            "home and housing" => Ok(Category::home_and_housing),
            "personal growth and development" => Ok(Category::personal_growth_and_development),
            "personal finance and budgeting" => Ok(Category::personal_finance_and_budgeting),
            "pets and animals" => Ok(Category::pets_and_animals),
            "philanthropic endeavors" => Ok(Category::philanthropic_endeavors),
            "political beliefs" => Ok(Category::political_beliefs),
            "shopping and fashion" => Ok(Category::shopping_and_fashion),
            "socializing" => Ok(Category::socializing),
            "spirituality and faith" => Ok(Category::spirituality_and_faith),
            "sports and games" => Ok(Category::sports_and_games),
            "technology and computing" => Ok(Category::technology_and_computing),
            "travel and exploration" => Ok(Category::travel_and_exploration),
            "transportation and vehicles" => Ok(Category::transportation_and_vehicles),
            _ => Err(CategoryError::InvalidStringValue(value.to_string())),
        }
    }
}



