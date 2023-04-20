use mime::Mime;
use scraper::{ElementRef, Selector};
use url::Url;

use crate::{
    error::MetaResult,
    meta::item::{MetadataItem, MetadataItemExt, MetadataItemExtractor, MetadataItemTarget},
};

use super::OgpMetadata;

pub enum OgpAudioItem {
    Url,
    SecureUrl,
    Mime,
}

impl MetadataItem for OgpAudioItem {
    fn selector(&self) -> MetaResult<Selector> {
        let selector = match self {
            Self::Url => Selector::parse("meta[property=\"og:audio\"]")?,
            Self::SecureUrl => Selector::parse("meta[property=\"og:audio:secure_url\"]")?,
            Self::Mime => Selector::parse("meta[property=\"og:audio:type\"]")?,
        };

        Ok(selector)
    }

    fn target(&self) -> MetadataItemTarget {
        MetadataItemTarget::Attr("content")
    }
}

impl MetadataItemExt for OgpAudioItem {}

impl MetadataItemExtractor<OgpMetadata> for OgpAudioItem {
    fn extract_item_to(&mut self, el: &ElementRef, target: &mut OgpMetadata) -> MetaResult<()> {
        match self {
            Self::Url => {
                let url = self.extract(el)?;
                target.audio.url = Some(url);
            }
            Self::SecureUrl => {
                let url = self.extract(el)?;
                target.audio.secure_url = Some(url);
            }
            Self::Mime => {
                let mime = self.extract(el)?;
                target.audio.mime = Some(mime);
            }
        }

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct OgpAudio {
    /// A URL to an audio file to accompany this object.
    pub url: Option<Url>,
    /// An alternate url to use if the webpage requires HTTPS.
    pub secure_url: Option<Url>,
    /// A MIME type for this audio.
    pub mime: Option<Mime>,
}
