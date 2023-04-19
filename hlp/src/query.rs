use crate::error::HlpResult;
use scraper::{Html, Selector};

pub fn calculate_key(title: &str) -> String {
    // TODO add author to key if found
    let key = title.to_ascii_lowercase();
    let mut splitted_key = key.trim().split_whitespace().enumerate();

    let mut key = String::new();
    while let Some((counter, part)) = splitted_key.next() {
        key.push_str(part);
        key.push('-');

        if counter == 3 {
            break;
        }
    }
    key.pop();
    key
}

pub fn find_title(html: &Html) -> HlpResult<Option<String>> {
    // TODO selector for <title>
    let selector = Selector::parse("h1")?;

    let title = html.select(&selector).next().map(|el| el.inner_html());
    Ok(title)
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct HtmlQueryReport {
    pub key: Option<String>,
    pub title: Option<String>,
}

impl HtmlQueryReport {
    pub fn from(html: &Html) -> HlpResult<Self> {
        let title = find_title(html)?;
        let key = title.as_ref().map(|title| calculate_key(title));

        Ok(Self { title, key })
    }
}

#[cfg(test)]
mod test {
    use scraper::Html;

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
            title: Some("Example Domain".to_owned()),
            key: Some("example-domain".to_owned()),
        };
        assert_eq!(report, expected);
    }
}
