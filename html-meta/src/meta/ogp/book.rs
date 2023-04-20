use crate::{
    error::MetaResult,
    meta::{
        item::{MetadataItem, MetadataItemExt, MetadataItemExtractor, MetadataItemTarget},
        value::{DateIso8601, Name},
    },
};
use isbn2::Isbn;
use scraper::{ElementRef, Selector};

use super::OgpMetadata;

pub enum OgpBookItem {
    Check,
    Author,
    Isbn,
    ReleaseDate,
    Tag,
}

impl MetadataItem for OgpBookItem {
    fn selector(&self) -> MetaResult<Selector> {
        let selector = match self {
            Self::Check => Selector::parse("meta[property=\"og:type\"][content=\"book\"]")?,
            Self::Author => Selector::parse("meta[property=\"og:book:author\"]")?,
            Self::Isbn => Selector::parse("meta[property=\"og:book:isbn\"]")?,
            Self::ReleaseDate => Selector::parse("meta[property=\"og:book:release_date\"]")?,
            Self::Tag => Selector::parse("meta[property=\"og:book:tag\"]")?,
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

impl MetadataItemExt for OgpBookItem {}

impl MetadataItemExtractor<OgpMetadata> for OgpBookItem {
    fn extract_item_to(&mut self, el: &ElementRef, target: &mut OgpMetadata) -> MetaResult<()> {
        match self {
            Self::Check => target.book.is_book = true,
            Self::Author => {
                let author = self.extract(el)?;
                target.book.authors.push(author);
            }
            Self::Isbn => {
                let isbn = self.extract(el)?;
                target.book.isbn = Some(isbn);
            }
            Self::ReleaseDate => {
                let date = self.extract(el)?;
                target.book.release_date = Some(date);
            }
            Self::Tag => {
                let tag = self.extract(el)?;
                target.book.tags.push(tag);
            }
        }

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct OgpBook {
    pub is_book: bool,
    /// Who wrote this book.
    pub authors: Vec<Name>,
    /// The ISBN
    pub isbn: Option<Isbn>,
    /// The date the book was released.
    pub release_date: Option<DateIso8601>,
    /// Tag words associated with this book.
    pub tags: Vec<String>,
}
