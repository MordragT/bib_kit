use crate::{error::MetaResult, meta::value::Title};
use scraper::{Html, Selector};

pub fn find_title(html: &Html) -> MetaResult<Option<String>> {
    // TODO selector for <title>
    let selector = Selector::parse("h1")?;

    let title = html
        .select(&selector)
        .next()
        .map(|el| el.text().next())
        .flatten()
        .map(ToOwned::to_owned);
    Ok(title)
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct HtmlQueryReport {
    pub title: Option<Title>,
}

impl HtmlQueryReport {
    pub fn from(html: &Html) -> MetaResult<Self> {
        let title = find_title(html)?;
        let title = title.map(Title::from);

        Ok(Self { title })
    }
}

#[cfg(test)]
mod test {
    use scraper::Html;

    use crate::meta::value::Title;

    use super::HtmlQueryReport;

    const EXAMPLE_HTML: &'static str = r#"<html>
    <head>
        <title>Example Domain</title>
    
        <meta charset="utf-8" />
        <meta http-equiv="Content-type" content="text/html; charset=utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <style type="text/css">
        body {
            background-color: #f0f0f2;
            margin: 0;
            padding: 0;
            font-family: -apple-system, system-ui, BlinkMacSystemFont, "Segoe UI", "Open Sans", "Helvetica Neue", Helvetica, Arial, sans-serif;
            
        }
        div {
            width: 600px;
            margin: 5em auto;
            padding: 2em;
            background-color: #fdfdff;
            border-radius: 0.5em;
            box-shadow: 2px 3px 7px 2px rgba(0,0,0,0.02);
        }
        a:link, a:visited {
            color: #38488f;
            text-decoration: none;
        }
        @media (max-width: 700px) {
            div {
                margin: 0 auto;
                width: auto;
            }
        }
        </style>    
    </head>
    
    <body>
    <div>
        <h1>Example Domain</h1>
        <p>This domain is for use in illustrative examples in documents. You may use this
        domain in literature without prior coordination or asking for permission.</p>
        <p><a href="https://www.iana.org/domains/example">More information...</a></p>
    </div>
    </body>
    </html>"#;

    #[test]
    fn html_query_report_query() {
        let html = Html::parse_document(EXAMPLE_HTML);

        let report = HtmlQueryReport::from(&html).unwrap();

        let expected = HtmlQueryReport {
            title: Some(Title::new("Example Domain")),
        };
        assert_eq!(report, expected);
    }
}
