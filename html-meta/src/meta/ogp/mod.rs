use super::{
    extractor::MetadataExtractor,
    item::{MetadataItem, MetadataItemExt, MetadataItemExtractor, MetadataItemTarget},
    value::Title,
};
use crate::error::MetaResult;
use scraper::{html::Select, ElementRef, Selector};
use std::fmt::Debug;
use unic_langid::LanguageIdentifier;
use url::Url;

pub use article::*;
pub use audio::*;
pub use book::*;
pub use image::*;
pub use profile::*;
pub use video::*;

mod article;
mod audio;
mod book;
mod image;
mod profile;
mod video;

#[derive(Default, Debug)]
pub struct OgpMetadata {
    /// The title of your object as it should appear within the graph, e.g., "The Rock".
    pub title: Option<Title>,
    /// The type of your object, e.g., "video.movie". Depending on the type you specify, other properties may also be required.
    pub kind: Option<String>,
    pub image: OgpImage,
    /// The canonical URL of your object that will be used as its permanent ID in the graph, e.g., "https://www.imdb.com/title/tt0117500/".
    pub url: Option<Url>,
    pub audio: OgpAudio,
    /// A one to two sentence description of your object.
    pub description: Option<String>,
    /// The locale these tags are marked up in. Of the format language_TERRITORY. Default is en_US.
    pub locale: Option<LanguageIdentifier>,
    /// Other locales this page is available in.
    pub alternate_locales: Vec<LanguageIdentifier>,
    /// If your object is part of a larger web site, the name which should be displayed for the overall site. e.g., "IMDb".
    pub site_name: Option<String>,
    pub video: OgpVideo,
    pub article: OgpArticle,
    pub book: OgpBook,
    pub profile: OgpProfile,
}

impl OgpMetadata {
    pub fn extract(select: Select) -> Self {
        let mut target = Self::default();

        let items = [
            OgpMetadataItem::Title.to_box(),
            OgpMetadataItem::Kind.to_box(),
            OgpMetadataItem::Url.to_box(),
            OgpMetadataItem::Description.to_box(),
            OgpMetadataItem::Locale.to_box(),
            OgpMetadataItem::AlternateLocale.to_box(),
            OgpMetadataItem::SiteName.to_box(),
            OgpArticleItem::Check.to_box(),
            OgpArticleItem::Author.to_box(),
            OgpArticleItem::ExpirationTime.to_box(),
            OgpArticleItem::ModifiedTime.to_box(),
            OgpArticleItem::PublishedTime.to_box(),
            OgpArticleItem::Section.to_box(),
            OgpArticleItem::Tag.to_box(),
            OgpAudioItem::Mime.to_box(),
            OgpAudioItem::SecureUrl.to_box(),
            OgpAudioItem::Url.to_box(),
            OgpBookItem::Check.to_box(),
            OgpBookItem::Author.to_box(),
            OgpBookItem::Isbn.to_box(),
            OgpBookItem::ReleaseDate.to_box(),
            OgpBookItem::Tag.to_box(),
            OgpImageItem::Alt.to_box(),
            OgpImageItem::Height.to_box(),
            OgpImageItem::Width.to_box(),
            OgpImageItem::Mime.to_box(),
            OgpImageItem::SecureUrl.to_box(),
            OgpImageItem::Url.to_box(),
            OgpProfileItem::FirstName.to_box(),
            OgpProfileItem::Gender.to_box(),
            OgpProfileItem::LastName.to_box(),
            OgpProfileItem::Username.to_box(),
            OgpVideoItem::Alt.to_box(),
            OgpVideoItem::Height.to_box(),
            OgpVideoItem::Width.to_box(),
            OgpVideoItem::Mime.to_box(),
            OgpVideoItem::SecureUrl.to_box(),
            OgpVideoItem::Url.to_box(),
        ];

        let mut extractor = MetadataExtractor::new();

        let errors = extractor.extract_to(select, items, &mut target);
        errors.print_failure();

        target
    }
}

#[derive(Debug, Clone, Copy)]
pub enum OgpMetadataItem {
    Title,
    Kind,
    Url,
    Description,
    Locale,
    AlternateLocale,
    SiteName,
}

impl MetadataItem for OgpMetadataItem {
    fn selector(&self) -> MetaResult<Selector> {
        let selector = match self {
            Self::Title => Selector::parse("meta[property=\"og:title\"]"),
            Self::Kind => Selector::parse("meta[property=\"og:type\"]"),
            Self::Url => Selector::parse("meta[property=\"og:url\"]"),
            Self::Description => Selector::parse("meta[property=\"og:description\"]"),
            Self::Locale => Selector::parse("meta[property=\"og:locale\"]"),
            Self::AlternateLocale => Selector::parse("meta[property=\"og:locale:alternate\"]"),
            Self::SiteName => Selector::parse("meta[property=\"og:site_name\"]"),
        }?;

        Ok(selector)
    }

    fn target(&self) -> MetadataItemTarget {
        MetadataItemTarget::Attr("content")
    }
}

impl MetadataItemExt for OgpMetadataItem {}

impl MetadataItemExtractor<OgpMetadata> for OgpMetadataItem {
    fn extract_item_to(&mut self, el: &ElementRef, target: &mut OgpMetadata) -> MetaResult<()> {
        match self {
            Self::Title => target.title = Some(self.extract::<Title>(el)?),
            Self::Kind => target.kind = Some(self.extract::<String>(el)?),
            Self::Url => target.url = Some(self.extract::<Url>(el)?),
            Self::Description => target.description = Some(self.extract::<String>(el)?),
            Self::Locale => target.locale = Some(self.extract::<LanguageIdentifier>(el)?),
            Self::AlternateLocale => target
                .alternate_locales
                .push(self.extract::<LanguageIdentifier>(el)?),
            Self::SiteName => target.site_name = Some(self.extract::<String>(el)?),
        }

        Ok(())
    }
}
