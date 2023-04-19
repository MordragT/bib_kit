#![feature(let_chains)]

use citation::CitationBuilder;
use doc::Document;
use hayagriva::{io::to_yaml_str, Entry};
use meta::{generic::GenericMetadata, ogp::OgpMetadata};
use query::HtmlQueryReport;
use wasm_bindgen::{prelude::wasm_bindgen, JsError};

pub mod citation;
pub mod date;
pub mod doc;
pub mod error;
pub mod meta;
pub mod query;

#[wasm_bindgen]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn generate_citation(dom: &str, url: &str) -> Result<String, JsError> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let doc = Document::parse(dom, url)?;
    let report = doc.data_report()?;
    let citation = CitationBuilder::from(report);
    let entry = Entry::from(citation);

    Ok(to_yaml_str(Some(&entry)).unwrap())
}

pub struct DataReport {
    pub generic_metadata: GenericMetadata,
    pub ogp_metadata: OgpMetadata,
    pub html_query_report: HtmlQueryReport,
}
