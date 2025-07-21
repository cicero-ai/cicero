/// Get
pub fn get(country: &str, region: &str) -> String {

    let name = match country.to_uppercase().as_str() {
        "AU" => AU(region),
        "CA" => CA(region),
        "DE" => DE(region),
        "US" => US(region),
        _ => "".to_string()
    };

    name
}


fn AU(region: &str) -> String{

    let name = match region.to_uppercase().as_str() {
        "ACT" => "Australian Capital Territory",
        "NSW" => "New South Wales",
        "NT" => "Northern Territory",
        "QLD" => "Queensland",
        "SA" => "South Australia",
        "TAS" => "Tasmania",
        "VIC" => "Victoria",
        "WA" => "Western Australia",
        _ => ""
    };

    name.to_string()
}

fn CA(region: &str) -> String{

    let name = match region.to_uppercase().as_str() {
        "AB" => "Alberta",
        "BC" => "British Columbia",
        "MB" => "Manitoba",
        "NB" => "New Brunswick",
        "NL" => "Newfoundland and Labrador",
        "NT" => "Northwest Territories",
        "NS" => "Nova Scotia",
        "NU" => "Nunavut",
        "ON" => "Ontario",
        "PE" => "Prince Edward Island",
        "QC" => "Quebec",
        "SK" => "Saskatchewan",
        "YT" => "Yukon",
        _ => ""
    };

    name.to_string()
}

fn DE(region: &str) -> String{

    let name = match region.to_uppercase().as_str() {
        "BW" => "Baden-Württemberg",
        "BY" => "Bayern (Bavaria)",
        "BE" => "Berlin",
        "BB" => "Brandenburg",
        "HB" => "Bremen",
        "HH" => "Hamburg",
        "HE" => "Hessen",
        "MV" => "Mecklenburg-Vorpommern",
        "NI" => "Niedersachsen (Lower Saxony)",
        "NRW" => "Nordrhein-Westfalen (North Rhine-Westphalia)",
        "RP" => "Rheinland-Pfalz (Rhineland-Palatinate)",
        "SL" => "Saarland",
        "SN" => "Sachsen (Saxony)",
        "ST" => "Sachsen-Anhalt (Saxony-Anhalt)",
        "SH" => "Schleswig-Holstein",
        "TH" => "Thüringen (Thuringia)",
        _ => ""
    };

    name.to_string()
}

fn US(region: &str) -> String{

    let name = match region.to_uppercase().as_str() {
        "AL" => "Alabama",
        "AK" => "Alaska",
        "AZ" => "Arizona",
        "AR" => "Arkansas",
        "CA" => "California",
        "CO" => "Colorado",
        "CT" => "Connecticut",
        "DE" => "Delaware",
        "FL" => "Florida",
        "GA" => "Georgia",
        "HI" => "Hawaii",
        "ID" => "Idaho",
        "IL" => "Illinois",
        "IN" => "Indiana",
        "IA" => "Iowa",
        "KS" => "Kansas",
        "KY" => "Kentucky",
        "LA" => "Louisiana",
        "ME" => "Maine",
        "MD" => "Maryland",
        "MA" => "Massachusetts",
        "MI" => "Michigan",
        "MN" => "Minnesota",
        "MS" => "Mississippi",
        "MO" => "Missouri",
        "MT" => "Montana",
        "NE" => "Nebraska",
        "NV" => "Nevada",
        "NH" => "New Hampshire",
        "NJ" => "New Jersey",
        "NM" => "New Mexico",
        "NY" => "New York",
        "NC" => "North Carolina",
        "ND" => "North Dakota",
        "OH" => "Ohio",
        "OK" => "Oklahoma",
        "OR" => "Oregon",
        "PA" => "Pennsylvania",
        "RI" => "Rhode Island",
        "SC" => "South Carolina",
        "SD" => "South Dakota",
        "TN" => "Tennessee",
        "TX" => "Texas",
        "UT" => "Utah",
        "VT" => "Vermont",
        "VA" => "Virginia",
        "WA" => "Washington",
        "WV" => "West Virginia",
        "WI" => "Wisconsin",
        "WY" => "Wyoming",
        _ => ""
    };

    name.to_string()
}


