use scraper::{html::Select, ElementRef, Selector};
use std::fmt::Debug;
use url::Url;

use super::{
    extractor::MetadataExtractor,
    item::{MetadataItem, MetadataItemTarget},
    value::MetadataValue,
};
use crate::error::{HlpError, HlpResult};

pub use article::OgpArticle;
pub use audio::OgpAudio;
pub use book::OgpBook;
pub use image::OgpImage;
pub use profile::OgpProfile;
pub use video::OgpVideo;

mod article;
mod audio;
mod book;
mod image;
mod profile;
mod video;

#[derive(Default, Debug)]
pub struct OgpMetadata {
    /// The title of your object as it should appear within the graph, e.g., "The Rock".
    pub title: Option<String>,
    /// The type of your object, e.g., "video.movie". Depending on the type you specify, other properties may also be required.
    pub kind: Option<String>,
    pub images: Vec<OgpImage>,
    /// The canonical URL of your object that will be used as its permanent ID in the graph, e.g., "https://www.imdb.com/title/tt0117500/".
    pub url: Option<Url>,
    pub audios: Vec<OgpAudio>,
    /// A one to two sentence description of your object.
    pub description: Option<String>,
    /// The locale these tags are marked up in. Of the format language_TERRITORY. Default is en_US.
    pub locale: Option<String>,
    /// Other locales this page is available in.
    pub alternate_locales: Vec<String>,
    /// If your object is part of a larger web site, the name which should be displayed for the overall site. e.g., "IMDb".
    pub site_name: Option<String>,
    pub videos: Vec<OgpVideo>,
    pub articles: Vec<OgpArticle>,
    pub books: Vec<OgpBook>,
    pub profiles: Vec<OgpProfile>,
}

fn to_target(value: MetadataValue, item: &OgpMetadataItem, target: &mut OgpMetadata) {
    // TODO validate that values are not None
    match item {
        OgpMetadataItem::Title => target.title = value.into_string(),
        OgpMetadataItem::Kind => target.kind = value.into_string(),
        OgpMetadataItem::Url => target.url = value.into_url(),
        OgpMetadataItem::Description => target.description = value.into_string(),
        OgpMetadataItem::Locale => target.locale = value.into_string(),
        OgpMetadataItem::AlternateLocale => {
            target.alternate_locales.push(value.into_string().unwrap())
        }
        OgpMetadataItem::SiteName => target.site_name = value.into_string(),
        _ => (),
    }
}

impl OgpMetadata {
    pub fn extract(select: Select) -> HlpResult<Self> {
        let mut this = Self::default();

        let mut extractor = MetadataExtractor::new();
        extractor.add(OgpMetadataItem::Title);
        extractor.add(OgpMetadataItem::Kind);
        extractor.add(OgpMetadataItem::Url);
        extractor.add(OgpMetadataItem::Description);
        extractor.add(OgpMetadataItem::Locale);
        extractor.add(OgpMetadataItem::AlternateLocale);
        extractor.add(OgpMetadataItem::SiteName);

        for el in select {
            extractor.extract_element_to(&el, &mut this, to_target)?;
        }
        Ok(this)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum OgpMetadataItem {
    Title,
    Kind,
    Image,
    Url,
    Audio,
    Description,
    Locale,
    AlternateLocale,
    SiteName,
    Video,
    Article,
    Book,
    Profile,
}

impl MetadataItem for OgpMetadataItem {
    fn selector(&self) -> HlpResult<Selector> {
        let selector = match self {
            Self::Title => Selector::parse("meta[property=\"og:title\"]"),
            Self::Kind => Selector::parse("meta[property=\"og:type\"]"),
            Self::Image => Selector::parse("meta[property=\"og:image\"]"),
            Self::Url => Selector::parse("meta[property=\"og:url\"]"),
            Self::Audio => Selector::parse("meta[property=\"og:audio\"]"),
            Self::Description => Selector::parse("meta[property=\"og:description\"]"),
            Self::Locale => Selector::parse("meta[property=\"og:locale\"]"),
            Self::AlternateLocale => Selector::parse("meta[property=\"og:locale:alternate\"]"),
            Self::SiteName => Selector::parse("meta[property=\"og:site_name\"]"),
            Self::Video => Selector::parse("meta[property=\"og:video\"]"),
            Self::Article => Selector::parse("meta[property=\"og:type\"][content=\"article\"]"),
            Self::Book => Selector::parse("meta[property=\"og:type\"][content=\"book\"]"),
            Self::Profile => Selector::parse("meta[property=\"og:type\"][content=\"profile\"]"),
        }?;

        Ok(selector)
    }

    fn target(&self) -> MetadataItemTarget {
        match self {
            Self::Article => MetadataItemTarget::Child(vec![]),
            Self::Book => MetadataItemTarget::Child(vec![]),
            Self::Profile => MetadataItemTarget::Child(vec![]),
            _ => MetadataItemTarget::Attr("content"),
        }
    }

    fn extract(&self, el: &ElementRef) -> HlpResult<MetadataValue> {
        let target = self.target();

        if let MetadataItemTarget::Attr(attr) = target {
            let content = el
                .value()
                .attr(attr)
                .ok_or(HlpError::ExtractionContentNotFound)?;

            match self {
                Self::Title
                | Self::Kind
                | Self::Description
                | Self::Locale
                | Self::AlternateLocale
                | Self::SiteName => Ok(content.to_owned().into()),
                Self::Url | Self::Image | Self::Video | Self::Audio => {
                    Ok(content.parse::<Url>()?.into())
                }
                Self::Article | Self::Book | Self::Profile => unreachable!(),
            }
        } else {
            Ok(MetadataValue::None)
        }
    }
}
