
use serde_derive::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Availability {
    Immediately,
    OneMinute,
    AvailableIn(AvailableTimePeriod),
    ConfirmIn(AvailableTimePeriod),
    LongTermRainCheck,
    SpecificTime(DateTime<utc>),
    Never}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AvailableTimePeriod {
    Minutes,
    Hours,
    Days,
    Weeks,
    SpecificTime(DateTime<Utc>)
}

