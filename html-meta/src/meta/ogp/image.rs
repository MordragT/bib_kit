use super::OgpMetadata;
use crate::{
    error::MetaResult,
    meta::item::{MetadataItem, MetadataItemExt, MetadataItemExtractor, MetadataItemTarget},
};
use mime::Mime;
use scraper::{ElementRef, Selector};
use url::Url;

pub enum OgpImageItem {
    Url,
    SecureUrl,
    Mime,
    Width,
    Height,
    Alt,
}

impl MetadataItem for OgpImageItem {
    fn selector(&self) -> MetaResult<Selector> {
        let selector = match self {
            Self::Url => Selector::parse("meta[property=\"og:image\"]")?,
            Self::SecureUrl => Selector::parse("meta[property=\"og:image:secure_url\"]")?,
            Self::Mime => Selector::parse("meta[property=\"og:image:type\"]")?,
            Self::Width => Selector::parse("meta[property=\"og:image:width\"]")?,
            Self::Height => Selector::parse("meta[property=\"og:image:height\"]")?,
            Self::Alt => Selector::parse("meta[property=\"og:image:alt\"]")?,
        };

        Ok(selector)
    }

    fn target(&self) -> MetadataItemTarget {
        MetadataItemTarget::Attr("content")
    }
}

impl MetadataItemExt for OgpImageItem {}

impl MetadataItemExtractor<OgpMetadata> for OgpImageItem {
    fn extract_item_to(&mut self, el: &ElementRef, target: &mut OgpMetadata) -> MetaResult<()> {
        match self {
            Self::Url => {
                let url = self.extract(el)?;
                target.image.url = Some(url);
            }
            Self::SecureUrl => {
                let url = self.extract(el)?;
                target.image.secure_url = Some(url);
            }
            Self::Mime => {
                let mime = self.extract(el)?;
                target.image.mime = Some(mime);
            }
            Self::Width => {
                let width = self.extract(el)?;
                target.image.width = Some(width);
            }
            Self::Height => {
                let height = self.extract(el)?;
                target.image.height = Some(height);
            }
            Self::Alt => {
                let alt = self.extract(el)?;
                target.image.alt = Some(alt);
            }
        }

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct OgpImage {
    /// An image URL which should represent your object within the graph.
    pub url: Option<Url>,
    /// An alternate url to use if the webpage requires HTTPS.
    pub secure_url: Option<Url>,
    /// A MIME type for this image.
    pub mime: Option<Mime>,
    /// The number of pixels wide.
    pub width: Option<u32>,
    /// The number of pixels high.
    pub height: Option<u32>,
    /// A description of what is in the image (not a caption). If the page specifies an og:image it should specify og:image:alt.
    pub alt: Option<String>,
}
