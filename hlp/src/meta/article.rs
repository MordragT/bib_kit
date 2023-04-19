use scraper::{ElementRef, Selector};

use crate::{doc::Document, error::HlpResult, DateIso8601};

pub enum OgpArticleKind {
    PublishedTime,
    ModifiedTime,
    ExpirationTime,
    Author,
    Section,
    Tag,
}

impl OgpArticleKind {
    pub fn matches(&self, el: &ElementRef) -> HlpResult<bool> {
        let selector = self.selector()?;
        Ok(selector.matches(el))
    }

    pub fn selector(&self) -> HlpResult<Selector> {
        let selector = match self {
            Self::PublishedTime => Selector::parse("meta[property=\"article:published_time\"]"),
            Self::ModifiedTime => Selector::parse("meta[property=\"article:modified_time\"]"),
            Self::ExpirationTime => Selector::parse("meta[property=\"article:expiration_time\"]"),
            Self::Author => Selector::parse(
                "meta[property=\"article:author\"], meta[property=\"article:author:name\"]",
            ),
            Self::Section => Selector::parse("meta[property=\"article:section\"]"),
            Self::Tag => Selector::parse(
                "meta[property=\"article:tag\"], meta[property=\"article:tag:name\"]",
            ),
        }?;

        Ok(selector)
    }
}

#[derive(Debug, Default)]
pub struct OgpArticle {
    /// When the article was first published.
    pub published_time: Option<DateIso8601>,
    /// When the article was last changed.
    pub modified_time: Option<DateIso8601>,
    /// When the article is out of date after.
    pub expiration_time: Option<DateIso8601>,
    /// Writers of the article.
    pub authors: Vec<String>,
    /// A high-level section name. E.g. Technology
    pub section: Option<String>,
    /// Tag words associated with this article.
    pub tags: Vec<String>,
}
