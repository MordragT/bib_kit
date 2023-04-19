use std::{collections::VecDeque, fmt::Debug, str::FromStr};

pub use article::OgpArticle;
pub use audio::OgpAudio;
pub use book::OgpBook;
pub use image::OgpImage;
use mime::Mime;
pub use profile::OgpProfile;
use scraper::{html::Select, node::Element, ElementRef, Html, Selector};
use url::Url;
pub use video::OgpVideo;

use crate::error::{HlpError, HlpResult};

use self::article::OgpArticleKind;

mod article;
mod audio;
mod book;
mod image;
mod profile;
mod video;

pub type DynMetadataDesc<T> = Box<dyn MetadataDesc<T>>;

pub enum MetadataItemTarget<T> {
    Attr(&'static str),
    Child(Vec<DynMetadataDesc<T>>),
}

pub trait MetadataDesc<T> {
    fn matches(&self, el: &ElementRef) -> HlpResult<bool>;
    fn selector(&self) -> HlpResult<Selector>;
    fn target(&self) -> MetadataItemTarget<T>;

    fn bridge(&self, extractor: MetadataExtractor<T>, target: &mut T);
    // fn extract(&self, el: &ElementRef, target: &mut T) -> HlpResult<()>;
    // fn child(&self) -> Option<Box<dyn MetadataDesc<T>>>;
    fn box_dyn(self) -> Box<dyn MetadataDesc<T>>
    where
        Self: Sized,
    {
        let boxxed = Box::new(self) as Box<dyn MetadataDesc<T>>;
        boxxed
    }
}

pub struct MetadataExtractor<'a, 'b, T> {
    context: VecDeque<(DynMetadataDesc<T>, bool)>,
    select: Select<'a, 'b>,
}

impl<'a, 'b, T> MetadataExtractor<'a, 'b, T> {
    fn extract_item<R: FromStr>(attr: &str, el: &ElementRef) -> HlpResult<Option<R>>
    where
        <R as FromStr>::Err: Debug,
    {
        let content = el
            .value()
            .attr(attr)
            .ok_or(HlpError::ExtractionContentNotFound)?;
        let output = content.parse::<R>().ok();
        Ok(output)
    }

    pub fn new(select: Select) -> Self {
        let context = VecDeque::new();
        Self { context, select }
    }

    pub fn add(&mut self, desc: impl MetadataDesc<T> + 'static) {
        self.context.push_back((desc.box_dyn(), false));
    }

    pub fn extract<F, R>(&self, target: &mut T, mut assign_to_target: F) -> HlpResult<()>
    where F: FnMut(&mut T, Option<R>),
    R: FromStr,
    <R as FromStr>::Err: Debug {
        for el in self.select {
            for (desc, skip) in self.context.iter_mut() {
                if *skip { continue }

                if desc.matches(&el)? {
                    match desc.target() {
                        MetadataItemTarget::Attr(attr) => {
                            let res = Self::extract_item(attr, &el)?;
                            assign_to_target(target, res);
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

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

impl OgpMetadata {
    pub fn extract(select: Select) -> HlpResult<Self> {
        let mut this = Self::default();

        let mut context = VecDeque::new();
        context.push_back(OgpMetadataKind::Title.box_dyn());
        context.push_back(OgpMetadataKind::Kind.box_dyn());
        context.push_back(OgpMetadataKind::Image.box_dyn());
        context.push_back(OgpMetadataKind::Url.box_dyn());
        context.push_back(OgpMetadataKind::Audio.box_dyn());
        context.push_back(OgpMetadataKind::Description.box_dyn());
        context.push_back(OgpMetadataKind::Locale.box_dyn());
        context.push_back(OgpMetadataKind::AlternateLocale.box_dyn());
        context.push_back(OgpMetadataKind::SiteName.box_dyn());
        context.push_back(OgpMetadataKind::Video.box_dyn());
        context.push_back(OgpMetadataKind::Article.box_dyn());
        context.push_back(OgpMetadataKind::Book.box_dyn());
        context.push_back(OgpMetadataKind::Profile.box_dyn());

        for el in select {
            while let Some(desc) = context.pop_front() {
                if desc.matches(&el)? {
                    match desc.target() {
                        MetadataItemTarget::Attr(attr) =>
                    }
                    desc.extract(&el, &mut this);
                    if let Some(child) = desc.child() {
                        context.push_front(child);
                    }
                } else {
                }
            }
        }
        Ok(this)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum OgpMetadataKind {
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

impl MetadataDesc<OgpMetadata> for OgpMetadataKind {
    fn matches(&self, el: &ElementRef) -> HlpResult<bool> {
        let selector = self.selector()?;
        Ok(selector.matches(el))
    }

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

    fn target(&self) -> MetadataItemTarget<OgpMetadata> {
        match self {
            Self::Article => MetadataItemTarget::Child(vec![]),
            Self::Book => MetadataItemTarget::Child(vec![]),
            Self::Profile => MetadataItemTarget::Child(vec![]),
            _ => MetadataItemTarget::Attr("content"),
        }
    }

    fn extract(&self, el: &ElementRef, target: &mut OgpMetadata) -> HlpResult<()> {
        match self {
            Self::Title => {
                let title = extract_item(self, el)?;
                target.title = title;
            }
            _ => todo!(),
        }
        Ok(())
    }
}

pub struct GenericMetadata {
    pub title: Option<String>,
    pub content_type: Option<Mime>,
    pub description: Option<String>,
    pub keywords: Vec<String>,
    pub language: Option<String>,
    pub author: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum GenericMetadataKind {
    Title,
    ContentType,
    Description,
    Keyword,
    Language,
    Author,
}

impl MetadataDesc<GenericMetadata> for GenericMetadataKind {
    fn matches(&self, el: &ElementRef) -> HlpResult<bool> {
        let selector = self.selector()?;
        Ok(selector.matches(el))
    }

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

    fn attr_target(&self) -> Option<&str> {
        Some("content")
    }

    fn child(&self) -> Option<Box<dyn MetadataDesc<GenericMetadata>>> {
        None
    }

    fn extract(&self, el: &ElementRef, target: &mut GenericMetadata) -> HlpResult<()> {
        match self {
            Self::Title => {
                let title = extract_item(self, el)?;
                target.title = title;
            }
            Self::ContentType => {
                let content_type = extract_item(self, el)?;
                target.content_type = content_type;
            }
            Self::Description => {
                let description = extract_item(self, el)?;
                target.description = description;
            }
            Self::Keyword => {
                if let Some(keyword) = extract_item(self, el)? {
                    target.keywords.push(keyword)
                }
            }
            Self::Language => {
                let language = extract_item(self, el)?;
                target.language = language;
            }
            Self::Author => {
                let author = extract_item(self, el)?;
                target.author = author;
            }
        }
        Ok(())
    }
}
