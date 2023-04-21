use std::{convert::Infallible, num::ParseIntError};

use isbn2::IsbnError;
use scraper::error::SelectorErrorKind;
use thiserror::Error;
use unic_langid::LanguageIdentifierError;
use wasm_bindgen::prelude::*;

#[derive(Debug, Error)]
pub enum MetaError {
    #[error("Selector Error: {0}")]
    Selector(#[from] SelectorErrorKind<'static>),
    #[error("Yaml Parse Error")]
    YamlParse,
    #[error("Int Parse Error: {0}")]
    IntParse(#[from] ParseIntError),
    #[error("Date Parse Error: {0}")]
    DateParse(#[from] chrono::ParseError),
    #[error("Url Parse Error: {0}")]
    UrlParse(#[from] url::ParseError),
    #[error("Isbn Error: {0}")]
    Isbn(IsbnError),
    #[error("Cannot parse issn")]
    IssnParse,
    #[error("Language Identifier Error: {0}")]
    LanguageIdentifier(#[from] LanguageIdentifierError),
    #[error("Cannot parse name")]
    NameParse,
    #[error("Title not Found")]
    TitleNotFound,
    #[error("No extraction target")]
    NoExtractionTarget,
    #[error("Extraction content not found")]
    ExtractionContentNotFound,
    #[error("MimeFrom Str Error: {0}")]
    MimeFromStr(#[from] mime::FromStrError),
    #[error("Wrong metadata type")]
    WrongMetadataType,
    #[error("Missing metadata")]
    MissingMetadata,
    #[error("Infallible")]
    Infallible(#[from] Infallible),
}

impl Into<JsValue> for MetaError {
    fn into(self) -> JsValue {
        self.to_string().into()
    }
}

// impl From<MetaError> for JsValue {
//     fn from(err: MetaError) -> Self {
//         err.to_string().into()
//     }
// }

impl From<IsbnError> for MetaError {
    fn from(value: IsbnError) -> Self {
        Self::Isbn(value)
    }
}

#[derive(Debug, Default)]
pub struct Errors {
    errors: Vec<MetaError>,
}

impl Errors {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn err(&mut self, err: MetaError) {
        self.errors.push(err);
    }

    pub fn errors(&mut self, errors: &mut Errors) {
        self.errors.append(&mut errors.errors)
    }

    pub fn capture<T>(&mut self, result: MetaResult<T>) -> Option<T> {
        match result {
            Err(e) => {
                self.err(e);
                None
            }
            Ok(some) => Some(some),
        }
    }

    pub fn is_success(&self) -> bool {
        self.errors.len() == 0
    }

    pub fn print_failure(&self) {
        if !self.is_success() {
            for err in &self.errors {
                println!("{err:?}")
            }
        }
    }
}

pub type MetaResult<T> = Result<T, MetaError>;
