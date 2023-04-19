use super::value::MetadataValue;
use crate::error::HlpResult;
use scraper::{ElementRef, Selector};

pub enum MetadataItemTarget {
    Attr(&'static str),
    Child(Vec<DynMetadataItem>),
}

pub type DynMetadataItem = Box<dyn MetadataItem>;

pub trait MetadataItem {
    fn matches(&self, el: &ElementRef) -> HlpResult<bool>;
    fn selector(&self) -> HlpResult<Selector>;
    fn target(&self) -> MetadataItemTarget;
    fn extract(&self, el: &ElementRef) -> HlpResult<MetadataValue>;

    fn box_dyn(self) -> Box<dyn MetadataItem>
    where
        Self: Sized + 'static,
    {
        let boxxed = Box::new(self) as Box<dyn MetadataItem>;
        boxxed
    }
}