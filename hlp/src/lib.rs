#![feature(let_chains)]

use std::fmt::Debug;

use citation::CitationBuilder;
use doc::Document;
use hayagriva::{io::to_yaml_str, Entry};
use meta::{generic::GenericMetadata, ogp::OgpMetadata};
use query::HtmlQueryReport;
use url::Url;
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

#[derive(Debug, Default)]
pub struct DataReportBuilder {
    key: PriorityData<String>,
    kind: PriorityData<String>,
    title: PriorityData<String>,
    note: PriorityData<String>,
    language: PriorityData<String>,
    authors: PriorityData<Vec<String>>,
    // date: Vec<DateIso8601>,
    // publishier: Vec<String>,
    url: PriorityData<Url>,
    // language: Vec<String>,
}

impl DataReportBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_url(mut self, url: Url) -> Self {
        self.url.first = Some(url);
        self
    }

    pub fn with_generic_metadata(mut self, generic_metadata: GenericMetadata) -> Self {
        let GenericMetadata {
            title,
            content_type,
            description,
            keywords,
            language,
            author,
        } = generic_metadata;

        self.title.second = title;
        self.note.second = description;
        self.language.second = language;
        self.authors.second = author.map(|a| vec![a]);

        self
    }

    pub fn with_ogp_metadata(mut self, ogp_metadata: OgpMetadata) -> Self {
        let OgpMetadata {
            title,
            kind,
            images,
            url,
            audios,
            description,
            locale,
            alternate_locales,
            site_name,
            videos,
            articles,
            books,
            profiles,
        } = ogp_metadata;

        self.title.first = title;
        self.url.first = url;
        self.note.first = description;
        self.language.first = locale;

        self
    }

    pub fn with_html_query_report(mut self, html_query_report: HtmlQueryReport) -> Self {
        let HtmlQueryReport { title } = html_query_report;

        self.title.third = title;

        self
    }

    pub fn build(self) -> DataReport {
        let key = self.key.highest();
        let kind = self.kind.highest();
        let title = self.title.highest();
        let note = self.note.highest();
        let language = self.language.highest();
        let authors = self.authors.highest();
        let url = self.url.highest();

        DataReport {
            key,
            kind,
            title,
            note,
            language,
            authors,
            url,
        }
    }
}

pub struct DataReport {
    pub key: Option<String>,
    pub kind: Option<String>,
    pub title: Option<String>,
    pub note: Option<String>,
    pub language: Option<String>,
    pub authors: Option<Vec<String>>,
    pub url: Option<Url>,
}
