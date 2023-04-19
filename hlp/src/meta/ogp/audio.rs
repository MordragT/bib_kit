use scraper::Selector;

use crate::{doc::Document, error::HlpResult};

use mime::Mime;
use url::Url;

#[derive(Debug)]
pub struct OgpAudio {
    /// A URL to an audio file to accompany this object.
    pub url: Url,
    /// An alternate url to use if the webpage requires HTTPS.
    pub secure_url: Option<Url>,
    /// A MIME type for this audio.
    pub mime: Option<Mime>,
}
