use super::{
    extractor::MetadataExtractor,
    item::{MetadataItem, MetadataItemTarget},
    value::MetadataValue,
};
use crate::error::{HlpError, HlpResult};
use mime::Mime;
use scraper::{html::Select, ElementRef, Selector};

#[derive(Debug, Default)]
pub struct GenericMetadata {
    pub title: Option<String>,
    pub content_type: Option<Mime>,
    pub description: Option<String>,
    pub keywords: Vec<String>,
    pub language: Option<String>,
    pub author: Option<String>,
}

fn to_target(value: MetadataValue, item: &GenericMetadataItem, target: &mut GenericMetadata) {
    // TODO validate that values are not None
    match item {
        GenericMetadataItem::Author => target.author = value.into_string(),
        GenericMetadataItem::ContentType => target.content_type = value.into_mime(),
        GenericMetadataItem::Description => target.description = value.into_string(),
        GenericMetadataItem::Keyword => target.keywords.push(value.into_string().unwrap()),
        GenericMetadataItem::Language => target.language = value.into_string(),
        GenericMetadataItem::Title => target.title = value.into_string(),
    }
}

impl GenericMetadata {
    pub fn extract(select: Select) -> HlpResult<Self> {
        let mut this = Self::default();

        let mut extractor = MetadataExtractor::new();
        extractor.add(GenericMetadataItem::Author);
        extractor.add(GenericMetadataItem::ContentType);
        extractor.add(GenericMetadataItem::Description);
        extractor.add(GenericMetadataItem::Keyword);
        extractor.add(GenericMetadataItem::Language);
        extractor.add(GenericMetadataItem::Title);

        for el in select {
            extractor.extract_element_to(&el, &mut this, to_target)?;
        }
        Ok(this)
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
    fn selector(&self) -> HlpResult<Selector> {
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

    fn extract(&self, el: &ElementRef) -> HlpResult<MetadataValue> {
        let target = self.target();

        if let MetadataItemTarget::Attr(attr) = target {
            let content = el
                .value()
                .attr(attr)
                .ok_or(HlpError::ExtractionContentNotFound)?;

            match self {
                Self::Title | Self::Description | Self::Keyword | Self::Language | Self::Author => {
                    Ok(content.to_owned().into())
                }
                Self::ContentType => Ok(content.parse::<Mime>()?.into()),
            }
        } else {
            Ok(MetadataValue::None)
        }
    }
}
