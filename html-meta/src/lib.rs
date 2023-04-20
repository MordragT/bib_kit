#![feature(let_chains)]
#![feature(inherent_associated_types)]

use doc::Document;
use hayagriva::io::to_yaml_str;
use std::fmt::Debug;
use wasm_bindgen::{prelude::wasm_bindgen, JsError};

pub mod citation;
pub mod doc;
pub mod error;
pub mod meta;
pub mod query;

#[wasm_bindgen]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn generate_citation(dom: &str, url: &str) -> Result<String, JsError> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let doc = Document::parse(dom, url)?;
    let entry = doc.citation_entry()?;

    Ok(to_yaml_str(Some(&entry)).unwrap())
}

#[derive(Debug)]
pub struct PriorityData<T> {
    pub first: Option<T>,
    pub second: Option<T>,
    pub third: Option<T>,
}

impl<T> PriorityData<T> {
    pub fn highest(self) -> Option<T> {
        if self.first.is_some() {
            return self.first;
        }
        if self.second.is_some() {
            return self.second;
        }
        self.third
    }
}

impl<T> Default for PriorityData<T> {
    fn default() -> Self {
        Self {
            first: None,
            second: None,
            third: None,
        }
    }
}
