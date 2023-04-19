#![feature(let_chains)]

use std::str::FromStr;

use chrono::{DateTime, Datelike, FixedOffset, ParseError};
use doc::Document;
use hayagriva::{io::to_yaml_str, types::Date, Entry};
use wasm_bindgen::{prelude::wasm_bindgen, JsError};

pub mod doc;
pub mod error;
pub mod meta;

#[wasm_bindgen]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn generate_citation(dom: &str, url: &str) -> Result<String, JsError> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let doc = Document::parse(dom, url)?;
    let entry = Entry::try_from(doc)?;
    Ok(to_yaml_str(Some(&entry)).unwrap())
}

#[derive(Debug, Default)]
pub struct DateIso8601 {
    date_time: DateTime<FixedOffset>,
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
            day: Some(date_time.day() as u8),
            month: Some(date_time.month() as u8),
            year: date_time.year(),
        }
    }
}
