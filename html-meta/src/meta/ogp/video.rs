use mime::Mime;
use scraper::{ElementRef, Selector};
use url::Url;

use crate::{
    error::MetaResult,
    meta::item::{MetadataItem, MetadataItemExt, MetadataItemExtractor, MetadataItemTarget},
};

use super::OgpMetadata;

pub enum OgpVideoItem {
    Url,
    SecureUrl,
    Mime,
    Width,
    Height,
    Alt,
}

impl MetadataItem for OgpVideoItem {
    fn selector(&self) -> MetaResult<Selector> {
        let selector = match self {
            Self::Url => Selector::parse("meta[property=\"og:video\"]")?,
            Self::SecureUrl => Selector::parse("meta[property=\"og:video:secure_url\"]")?,
            Self::Mime => Selector::parse("meta[property=\"og:video:type\"]")?,
            Self::Width => Selector::parse("meta[property=\"og:video:width\"]")?,
            Self::Height => Selector::parse("meta[property=\"og:video:height\"]")?,
            Self::Alt => Selector::parse("meta[property=\"og:video:alt\"]")?,
        };

        Ok(selector)
    }

    fn target(&self) -> MetadataItemTarget {
        MetadataItemTarget::Attr("content")
    }
}

impl MetadataItemExt for OgpVideoItem {}

impl MetadataItemExtractor<OgpMetadata> for OgpVideoItem {
    fn extract_item_to(&mut self, el: &ElementRef, target: &mut OgpMetadata) -> MetaResult<()> {
        match self {
            Self::Url => {
                let url = self.extract(el)?;
                target.video.url = Some(url);
            }
            Self::SecureUrl => {
                let url = self.extract(el)?;
                target.video.secure_url = Some(url);
            }
            Self::Mime => {
                let mime = self.extract(el)?;
                target.video.mime = Some(mime);
            }
            Self::Width => {
                let width = self.extract(el)?;
                target.video.width = Some(width);
            }
            Self::Height => {
                let height = self.extract(el)?;
                target.video.height = Some(height);
            }
            Self::Alt => {
                let alt = self.extract(el)?;
                target.video.alt = Some(alt);
            }
        }

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct OgpVideo {
    /// A URL to a video file that complements this object.
    pub url: Option<Url>,
    /// An alternate url to use if the webpage requires HTTPS.
    pub secure_url: Option<Url>,
    /// A MIME type for this video.
    pub mime: Option<Mime>,
    /// The number of pixels wide.
    pub width: Option<u32>,
    /// The number of pixels high.
    pub height: Option<u32>,
    /// A description of what is in this video (not a caption).
    pub alt: Option<String>,
}
