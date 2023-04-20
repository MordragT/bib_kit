use std::str::FromStr;

use crate::error::{Errors, MetaError, MetaResult};
use scraper::{html::Select, ElementRef, Selector};

pub enum MetadataItemTarget {
    Attr(&'static str),
    None,
}

impl MetadataItemTarget {
    pub fn extract<T>(&self, el: &ElementRef) -> MetaResult<T>
    where
        T: FromStr,
        MetaError: From<<T as FromStr>::Err>,
    {
        if let Self::Attr(attr) = self {
            let content = el
                .value()
                .attr(attr)
                .ok_or(MetaError::ExtractionContentNotFound)?;

            let parsed = content.parse::<T>()?;

            Ok(parsed)
        } else {
            Err(MetaError::NoExtractionTarget)
        }
    }
}

pub trait MetadataItem {
    fn selector(&self) -> MetaResult<Selector>;
    fn target(&self) -> MetadataItemTarget;
    fn matches(&self, el: &ElementRef) -> MetaResult<bool> {
        let selector = self.selector()?;
        Ok(selector.matches(el))
    }
}

pub trait MetadataItemExt: MetadataItem {
    fn extract<T>(&self, el: &ElementRef) -> MetaResult<T>
    where
        T: FromStr,
        MetaError: From<<T as FromStr>::Err>,
    {
        self.target().extract::<T>(el)
    }
}

pub type MetadataItemExtractorBox<T> = Box<dyn MetadataItemExtractor<T>>;

pub trait MetadataItemExtractor<T>: MetadataItem {
    fn extract_item_to(&mut self, el: &ElementRef, target: &mut T) -> MetaResult<()>;

    fn extract_to(&mut self, select: Select, target: &mut T) -> Errors {
        let mut errors = Errors::new();

        for el in select {
            if let Some(matches) = errors.capture(self.matches(&el)) && matches {
                errors.capture(self.extract_item_to(&el, target));
            }
        }

        errors
    }

    fn to_box(self) -> MetadataItemExtractorBox<T>
    where
        Self: Sized + 'static,
    {
        Box::new(self) as MetadataItemExtractorBox<T>
    }
}
