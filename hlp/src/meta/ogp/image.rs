use mime::Mime;
use url::Url;

#[derive(Debug)]
pub struct OgpImage {
    /// An image URL which should represent your object within the graph.
    pub url: Url,
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
