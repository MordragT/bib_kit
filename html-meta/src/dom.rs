use crate::{
    error::MetaResult,
    meta::{generic::GenericMetadata, ogp::OgpMetadata},
    query::HtmlQueryReport,
};
use scraper::{Html, Selector};
use url::Url;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dom {
    html: Html,
    url: Url,
}

#[wasm_bindgen]
impl Dom {
    #[wasm_bindgen(constructor)]
    pub fn parse(dom: &str, url: &str) -> MetaResult<Dom> {
        let html = Html::parse_document(dom);
        let url = Url::parse(url)?;
        Ok(Self { html, url })
    }
}

impl Dom {
    pub fn generic_metadata(&self) -> MetaResult<GenericMetadata> {
        let selector = Selector::parse("meta")?;
        let select = self.html.select(&selector);

        Ok(GenericMetadata::extract(select))
    }

    pub fn ogp_metadata(&self) -> MetaResult<OgpMetadata> {
        let selector = Selector::parse("meta")?;
        let select = self.html.select(&selector);

        Ok(OgpMetadata::extract(select))
    }

    pub fn html_query_report(&self) -> MetaResult<HtmlQueryReport> {
        let report = HtmlQueryReport::from(&self.html)?;
        Ok(report)
    }

    pub fn url(&self) -> &Url {
        &self.url
    }
}
