use scraper::error::SelectorErrorKind;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HlpError {
    #[error("Selector Error: {0}")]
    Selector(#[from] SelectorErrorKind<'static>),
    #[error("Url Parse Error: {0}")]
    UrlParse(#[from] url::ParseError),
    #[error("Title not Found")]
    TitleNotFound,
    #[error("No extraction target")]
    NoExtractionTarget,
    #[error("Extraction content not found")]
    ExtractionContentNotFound,
    #[error("MimeFrom Str Error: {0}")]
    MimeFromStr(#[from] mime::FromStrError),
}

pub type HlpResult<T> = Result<T, HlpError>;
