use std::collections::VecDeque;

use scraper::ElementRef;

use super::{item::MetadataItem, value::MetadataValue};
use crate::error::HlpResult;

pub struct MetadataExtractionContext<I: MetadataItem> {
    item: I,
    skip: bool,
}

pub struct MetadataExtractor<I: MetadataItem> {
    context: VecDeque<MetadataExtractionContext<I>>,
}

impl<I: MetadataItem> MetadataExtractor<I> {
    pub fn extract_element_to<T, F>(
        &mut self,
        el: &ElementRef,
        target: &mut T,
        mut to_target: F,
    ) -> HlpResult<()>
    where
        F: FnMut(MetadataValue, &I, &mut T),
    {
        for MetadataExtractionContext { item, skip } in &mut self.context {
            if *skip {
                continue;
            }

            if item.matches(el)? {
                let value = item.extract(el)?;
                to_target(value, item, target);
                *skip = true;
            }
        }

        Ok(())
    }

    pub fn new() -> Self {
        let context = VecDeque::new();
        Self { context }
    }

    pub fn add(&mut self, item: I) {
        let context_item = MetadataExtractionContext { item, skip: false };
        self.context.push_back(context_item);
    }
}
