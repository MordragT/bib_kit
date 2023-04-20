use hayagriva::types::QualifiedUrl;
use isbn2::{Isbn10, Isbn13};
use mime::Mime;
use unic_langid::LanguageIdentifier;
use url::Url;

pub use date::DateIso8601;
pub use issn::Issn;
pub use name::Name;
pub use title::Title;

use crate::error::MetaError;

pub fn into_qualified(value: Url) -> QualifiedUrl {
    QualifiedUrl {
        value,
        visit_date: Some(DateIso8601::now().into()),
    }
}

mod date;
mod issn;
mod name;
mod title;

#[derive(Debug, Default)]
pub enum MetadataValue {
    Url(Url),
    Mime(Mime),
    Text(String),
    Date(DateIso8601),
    Isbn10(Isbn10),
    Isbn13(Isbn13),
    Issn(Issn),
    Language(LanguageIdentifier),
    Name(Name),
    Title(Title),
    #[default]
    None,
}

impl MetadataValue {
    pub fn into_url(self) -> Option<Url> {
        self.try_into().ok()
    }

    pub fn into_mime(self) -> Option<Mime> {
        self.try_into().ok()
    }

    pub fn into_text(self) -> Option<String> {
        self.try_into().ok()
    }

    pub fn into_date(self) -> Option<DateIso8601> {
        self.try_into().ok()
    }

    pub fn into_isbn10(self) -> Option<Isbn10> {
        self.try_into().ok()
    }

    pub fn into_isbn13(self) -> Option<Isbn13> {
        self.try_into().ok()
    }

    pub fn into_issn(self) -> Option<Issn> {
        self.try_into().ok()
    }

    pub fn into_language(self) -> Option<LanguageIdentifier> {
        self.try_into().ok()
    }

    pub fn into_name(self) -> Option<Name> {
        self.try_into().ok()
    }

    pub fn into_title(self) -> Option<Title> {
        self.try_into().ok()
    }
}

impl<T: Into<MetadataValue>> From<Option<T>> for MetadataValue {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(val) => val.into(),
            None => MetadataValue::None,
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
        Self::Text(value)
    }
}

impl From<DateIso8601> for MetadataValue {
    fn from(date: DateIso8601) -> Self {
        Self::Date(date)
    }
}

impl From<Isbn10> for MetadataValue {
    fn from(value: Isbn10) -> Self {
        Self::Isbn10(value)
    }
}

impl From<Isbn13> for MetadataValue {
    fn from(value: Isbn13) -> Self {
        Self::Isbn13(value)
    }
}

impl From<Title> for MetadataValue {
    fn from(value: Title) -> Self {
        Self::Title(value)
    }
}

impl From<Issn> for MetadataValue {
    fn from(value: Issn) -> Self {
        Self::Issn(value)
    }
}

impl From<Name> for MetadataValue {
    fn from(value: Name) -> Self {
        Self::Name(value)
    }
}

impl From<LanguageIdentifier> for MetadataValue {
    fn from(value: LanguageIdentifier) -> Self {
        Self::Language(value)
    }
}

impl TryFrom<MetadataValue> for Url {
    type Error = MetaError;

    fn try_from(value: MetadataValue) -> Result<Self, Self::Error> {
        match value {
            MetadataValue::Url(url) => Ok(url),
            _ => Err(MetaError::WrongMetadataType),
        }
    }
}

impl TryFrom<MetadataValue> for Mime {
    type Error = MetaError;

    fn try_from(value: MetadataValue) -> Result<Self, Self::Error> {
        match value {
            MetadataValue::Mime(mime) => Ok(mime),
            _ => Err(MetaError::WrongMetadataType),
        }
    }
}

impl TryFrom<MetadataValue> for String {
    type Error = MetaError;

    fn try_from(value: MetadataValue) -> Result<Self, Self::Error> {
        match value {
            MetadataValue::Text(text) => Ok(text),
            _ => Err(MetaError::WrongMetadataType),
        }
    }
}

impl TryFrom<MetadataValue> for DateIso8601 {
    type Error = MetaError;

    fn try_from(value: MetadataValue) -> Result<Self, Self::Error> {
        match value {
            MetadataValue::Date(date) => Ok(date),
            _ => Err(MetaError::WrongMetadataType),
        }
    }
}

impl TryFrom<MetadataValue> for Isbn10 {
    type Error = MetaError;

    fn try_from(value: MetadataValue) -> Result<Self, Self::Error> {
        match value {
            MetadataValue::Isbn10(isbn) => Ok(isbn),
            _ => Err(MetaError::WrongMetadataType),
        }
    }
}

impl TryFrom<MetadataValue> for Isbn13 {
    type Error = MetaError;

    fn try_from(value: MetadataValue) -> Result<Self, Self::Error> {
        match value {
            MetadataValue::Isbn13(isbn) => Ok(isbn),
            _ => Err(MetaError::WrongMetadataType),
        }
    }
}

impl TryFrom<MetadataValue> for Issn {
    type Error = MetaError;

    fn try_from(value: MetadataValue) -> Result<Self, Self::Error> {
        match value {
            MetadataValue::Issn(issn) => Ok(issn),
            _ => Err(MetaError::WrongMetadataType),
        }
    }
}

impl TryFrom<MetadataValue> for LanguageIdentifier {
    type Error = MetaError;

    fn try_from(value: MetadataValue) -> Result<Self, Self::Error> {
        match value {
            MetadataValue::Language(lang) => Ok(lang),
            _ => Err(MetaError::WrongMetadataType),
        }
    }
}

impl TryFrom<MetadataValue> for Name {
    type Error = MetaError;

    fn try_from(value: MetadataValue) -> Result<Self, Self::Error> {
        match value {
            MetadataValue::Name(name) => Ok(name),
            _ => Err(MetaError::WrongMetadataType),
        }
    }
}

impl TryFrom<MetadataValue> for Title {
    type Error = MetaError;

    fn try_from(value: MetadataValue) -> Result<Self, Self::Error> {
        match value {
            MetadataValue::Title(title) => Ok(title),
            _ => Err(MetaError::WrongMetadataType),
        }
    }
}
