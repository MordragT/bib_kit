use std::str::FromStr;

use chrono::{DateTime, Datelike, FixedOffset, Local, ParseError};
use hayagriva::types::Date;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct DateIso8601 {
    date_time: DateTime<FixedOffset>,
}

impl DateIso8601 {
    pub fn now() -> Self {
        let date_time = Local::now().into();

        Self { date_time }
    }
}

impl FromStr for DateIso8601 {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let date_time = DateTime::parse_from_rfc3339(s)?;
        Ok(Self { date_time })
    }
}

impl From<DateIso8601> for Date {
    fn from(date_iso_8601: DateIso8601) -> Self {
        let date_time = date_iso_8601.date_time;
        Date {
            day: Some(date_time.day() as u8 - 1),
            month: Some(date_time.month() as u8 - 1),
            year: date_time.year(),
        }
    }
}
