use scraper::error::SelectorErrorKind;
use thiserror::Error;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Error)]
pub enum HlpError {
    #[error("Selector Error: {0}")]
    Selector(#[from] SelectorErrorKind<'static>),
    #[error("Url Parse Error: {0}")]
    UrlParse(#[from] url::ParseError),
    #[error("Title not Found")]
    TitleNotFound,
}

pub type HlpResult<T> = Result<T, HlpError>;
