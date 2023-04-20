use super::{
    extractor::MetadataExtractor,
    item::{MetadataItem, MetadataItemExt, MetadataItemExtractor, MetadataItemTarget},
    value::{Name, Title},
};
use crate::error::MetaResult;
use mime::Mime;
use scraper::{html::Select, ElementRef, Selector};
use unic_langid::LanguageIdentifier;

#[derive(Debug, Default)]
pub struct GenericMetadata {
    pub title: Option<Title>,
    pub content_type: Option<Mime>,
    pub description: Option<String>,
    pub keywords: Vec<String>,
    pub language: Option<LanguageIdentifier>,
    pub author: Option<Name>,
}

impl GenericMetadata {
    pub fn extract(select: Select) -> Self {
        let mut target = Self::default();

        let items = [
            GenericMetadataItem::Author.to_box(),
            GenericMetadataItem::ContentType.to_box(),
            GenericMetadataItem::Description.to_box(),
            GenericMetadataItem::Keyword.to_box(),
            GenericMetadataItem::Language.to_box(),
            GenericMetadataItem::Title.to_box(),
        ];

        let mut extractor = MetadataExtractor::new();

        let errors = extractor.extract_to(select, items, &mut target);
        errors.print_failure();

        target
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GenericMetadataItem {
    Title,
    ContentType,
    Description,
    Keyword,
    Language,
    Author,
}

impl MetadataItem for GenericMetadataItem {
    fn selector(&self) -> MetaResult<Selector> {
        let selector = match self {
            Self::Title => Selector::parse("meta[name=\"title\"]"),
            Self::ContentType => Selector::parse("meta[name=\"content-type\"]"),
            Self::Description => Selector::parse("meta[name=\"description\"]"),
            Self::Keyword => Selector::parse("meta[name=\"keywords\"]"),
            Self::Language => {
                Selector::parse("meta[name=\"language\"], meta[http-equiv=\"content-language\"]")
            }
            Self::Author => Selector::parse("meta[name=\"author\"]"),
        }?;

        Ok(selector)
    }

    fn target(&self) -> MetadataItemTarget {
        MetadataItemTarget::Attr("content")
    }
}

impl MetadataItemExt for GenericMetadataItem {}

impl MetadataItemExtractor<GenericMetadata> for GenericMetadataItem {
    fn extract_item_to(&mut self, el: &ElementRef, target: &mut GenericMetadata) -> MetaResult<()> {
        match self {
            Self::Title => target.title = Some(self.extract::<Title>(el)?),
            Self::Language => target.language = Some(self.extract::<LanguageIdentifier>(el)?),
            Self::Author => target.author = Some(self.extract::<Name>(el)?),
            Self::Description => target.description = Some(self.extract::<String>(el)?),
            Self::Keyword => target.keywords.push(self.extract::<String>(el)?),
            Self::ContentType => target.content_type = Some(self.extract::<Mime>(el)?),
        }
        Ok(())
    }
}
