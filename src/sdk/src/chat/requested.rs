
use serde_derive::{Serialize, Deserialize};
use atlas_web::scraper::SocialNetwork;
use super::RelationshipType;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequestedItem {
    FirstName,
    LastName,
    FullName,
    BusinessName,
    Address,
    City,
    Region,
    Country,
    FullAddress,
    PhoneNumber,
    EmailAddress,
    OtherContactMethod((SocialNetwork, String)),
    URL,
    File(usize),
    SshKey,
    ServerAccess,
    SSmtpDetails,
    ApiKey,
    Relationship(RelationshipType),
    ProductName,
    ProductBrand,
    ProductModel,
    ProductOther,
    DateTime,
    Frequency,
    Date,
    Time,
    Year,
    Month,
    Day,
    DayOfWeek,
    DatePeriod,
    TimePeriod,
    Other
}




