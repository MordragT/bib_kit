use mime::Mime;
use url::Url;

use crate::date::DateIso8601;

pub enum MetadataValue {
    Url(Url),
    Mime(Mime),
    String(String),
    Date(DateIso8601),
    None,
}

impl MetadataValue {
    pub fn into_url(self) -> Option<Url> {
        match self {
            Self::Url(url) => Some(url),
            _ => None,
        }
    }

    pub fn into_mime(self) -> Option<Mime> {
        match self {
            Self::Mime(mime) => Some(mime),
            _ => None,
        }
    }

    pub fn into_string(self) -> Option<String> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn into_date(self) -> Option<DateIso8601> {
        match self {
            Self::Date(date) => Some(date),
            _ => None,
        }
    }
}

impl From<Url> for MetadataValue {
    fn from(url: Url) -> Self {
        Self::Url(url)
    }
}

impl From<Mime> for MetadataValue {
    fn from(mime: Mime) -> Self {
        Self::Mime(mime)
    }
}

impl From<String> for MetadataValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<DateIso8601> for MetadataValue {
    fn from(date: DateIso8601) -> Self {
        Self::Date(date)
    }
}
