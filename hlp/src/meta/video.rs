use mime::Mime;
use url::Url;

#[derive(Debug)]
pub struct OgpVideo {
    /// A URL to a video file that complements this object.
    pub url: Url,
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
