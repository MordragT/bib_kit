use crate::{
    citation::CitationBuilder,
    error::MetaResult,
    meta::{generic::GenericMetadata, ogp::OgpMetadata},
    query::HtmlQueryReport,
};
use hayagriva::Entry;
use scraper::{Html, Selector};
use url::Url;

pub struct Document {
    html: Html,
    url: Url,
}

impl Document {
    pub fn parse(raw: &str, url: &str) -> MetaResult<Self> {
        let html = Html::parse_document(raw);
        let url = Url::parse(url)?;
        Ok(Self { html, url })
    }

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

    pub fn citation_entry(&self) -> MetaResult<Entry> {
        let generic_metadata = self.generic_metadata()?;
        let ogp_metadata = self.ogp_metadata()?;
        let html_query_report = self.html_query_report()?;

        let report = CitationBuilder::new()
            .with_url(self.url.clone())
            .with_generic_metadata(generic_metadata)
            .with_ogp_metadata(ogp_metadata)
            .with_html_query_report(html_query_report)
            .build();

        Ok(report)
    }
}
