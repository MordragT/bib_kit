use crate::{
    error::HlpResult,
    meta::{generic::GenericMetadata, ogp::OgpMetadata},
    query::HtmlQueryReport,
    DataReport, DataReportBuilder,
};
use scraper::{Html, Selector};
use url::Url;

pub struct Document {
    html: Html,
    url: Url,
}

impl Document {
    pub fn parse(raw: &str, url: &str) -> HlpResult<Self> {
        let html = Html::parse_document(raw);
        let url = Url::parse(url)?;
        Ok(Self { html, url })
    }

    pub fn generic_metadata(&self) -> HlpResult<GenericMetadata> {
        let selector = Selector::parse("meta")?;
        let select = self.html.select(&selector);

        GenericMetadata::extract(select)
    }

    pub fn ogp_metadata(&self) -> HlpResult<OgpMetadata> {
        let selector = Selector::parse("meta")?;
        let select = self.html.select(&selector);

        OgpMetadata::extract(select)
    }

    pub fn html_query_report(&self) -> HlpResult<HtmlQueryReport> {
        let report = HtmlQueryReport::from(&self.html)?;
        Ok(report)
    }

    pub fn data_report(&self) -> HlpResult<DataReport> {
        let generic_metadata = self.generic_metadata()?;
        let ogp_metadata = self.ogp_metadata()?;
        let html_query_report = self.html_query_report()?;

        let report = DataReportBuilder::new()
            .with_url(self.url.clone())
            .with_generic_metadata(generic_metadata)
            .with_ogp_metadata(ogp_metadata)
            .with_html_query_report(html_query_report)
            .build();

        Ok(report)
    }
}
