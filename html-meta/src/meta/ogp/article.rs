use super::OgpMetadata;
use crate::{
    error::MetaResult,
    meta::{
        item::{MetadataItem, MetadataItemExt, MetadataItemExtractor, MetadataItemTarget},
        value::{DateIso8601, Name},
    },
};
use scraper::{ElementRef, Selector};

pub enum OgpArticleItem {
    Check,
    PublishedTime,
    ModifiedTime,
    ExpirationTime,
    Author,
    Section,
    Tag,
}

impl MetadataItem for OgpArticleItem {
    fn selector(&self) -> MetaResult<Selector> {
        let selector = match self {
            Self::Check => Selector::parse("meta[property=\"og:type\"][content=\"article\"]")?,
            Self::PublishedTime => Selector::parse("meta[property=\"og:article:published_time\"]")?,
            Self::ModifiedTime => Selector::parse("meta[property=\"og:article:modified_time\"]")?,
            Self::ExpirationTime => {
                Selector::parse("meta[property=\"og:article:expiration_time\"]")?
            }
            Self::Author => Selector::parse("meta[property=\"og:article:author\"]")?,
            Self::Section => Selector::parse("meta[property=\"og:article:section\"]")?,
            Self::Tag => Selector::parse("meta[property=\"og:article:tag\"]")?,
        };

        Ok(selector)
    }

    fn target(&self) -> MetadataItemTarget {
        match self {
            Self::Check => MetadataItemTarget::None,
            _ => MetadataItemTarget::Attr("content"),
        }
    }
}

impl MetadataItemExt for OgpArticleItem {}

impl MetadataItemExtractor<OgpMetadata> for OgpArticleItem {
    fn extract_item_to(&mut self, el: &ElementRef, target: &mut OgpMetadata) -> MetaResult<()> {
        match self {
            Self::Check => target.article.is_article = true,
            Self::PublishedTime => {
                let time = self.extract(el)?;
                target.article.published_time = Some(time);
            }
            Self::ModifiedTime => {
                let time = self.extract(el)?;
                target.article.modified_time = Some(time);
            }
            Self::ExpirationTime => {
                let time = self.extract(el)?;
                target.article.expiration_time = Some(time);
            }
            Self::Author => {
                let author = self.extract(el)?;
                target.article.authors.push(author);
            }
            Self::Tag => {
                let tag = self.extract(el)?;
                target.article.tags.push(tag);
            }
            Self::Section => {
                let section = self.extract(el)?;
                target.article.section = Some(section);
            }
        }

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct OgpArticle {
    pub is_article: bool,
    /// When the article was first published.
    pub published_time: Option<DateIso8601>,
    /// When the article was last changed.
    pub modified_time: Option<DateIso8601>,
    /// When the article is out of date after.
    pub expiration_time: Option<DateIso8601>,
    /// Writers of the article.
    pub authors: Vec<Name>,
    /// A high-level section name. E.g. Technology
    pub section: Option<String>,
    /// Tag words associated with this article.
    pub tags: Vec<String>,
}
