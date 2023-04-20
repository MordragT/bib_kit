use super::item::MetadataItemExtractorBox;
use crate::error::Errors;
use scraper::{html::Select, ElementRef};
use std::collections::VecDeque;

pub struct MetadataExtractionContext<T> {
    item: MetadataItemExtractorBox<T>,
    skip: bool,
}

pub struct MetadataExtractor<T> {
    context: VecDeque<MetadataExtractionContext<T>>,
}

impl<T> MetadataExtractor<T> {
    pub fn extract_element_to(&mut self, el: &ElementRef, target: &mut T) -> Errors {
        let mut errors = Errors::new();

        for MetadataExtractionContext { item, skip } in &mut self.context {
            if *skip {
                continue;
            }

            if let Some(matches) = errors.capture(item.matches(el)) && matches {
                errors.capture(item.extract_item_to(el, target));
                *skip = true;
            }
        }

        errors
    }

    pub fn extract_to(
        &mut self,
        select: Select,
        items: impl IntoIterator<Item = MetadataItemExtractorBox<T>>,
        target: &mut T,
    ) -> Errors {
        for item in items {
            self.add(item);
        }

        let mut errors = Errors::new();

        for el in select {
            let mut ele_errors = self.extract_element_to(&el, target);
            errors.errors(&mut ele_errors);
        }

        errors
    }

    pub fn new() -> Self {
        let context = VecDeque::new();
        Self { context }
    }

    pub fn add(&mut self, item: MetadataItemExtractorBox<T>) {
        let context_item = MetadataExtractionContext { item, skip: false };
        self.context.push_back(context_item);
    }
}
