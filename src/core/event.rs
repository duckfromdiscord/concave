use chrono::{DateTime, NaiveDate, Utc};
use either::Either;

#[derive(Clone, Debug)]
pub struct Location {
    pub coords: Option<String>,
    pub venue: Option<String>,
    pub loc: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Event {
    pub date: Either<DateTime<Utc>, NaiveDate>,
    pub name: String,
    pub location: Option<Location>,
}
