use crate::error::{HlpError, HlpResult};
use chrono::{Datelike, Utc};
use hayagriva::{
    types::{Date, EntryType, QualifiedUrl, Title},
    Entry,
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

    pub fn find_key(&self) -> HlpResult<String> {
        // TODO add author to key if found
        self.find_title().map(|title| {
            let mut key = title.canonical.value;
            key.make_ascii_lowercase();
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
        })
    }

    pub fn find_entry_type(&self) -> HlpResult<EntryType> {
        // TODO

        let entry_type = EntryType::Web;
        Ok(entry_type)
    }

    pub fn find_title(&self) -> HlpResult<Title> {
        // TODO selector for <title>

        let selector = Selector::parse("h1")?;

        let el = self
            .html
            .select(&selector)
            .next()
            .ok_or(HlpError::TitleNotFound)?;

        let title = Title::new(el.inner_html());
        Ok(title)
    }
}

fn now() -> Date {
    let now = Utc::now();
    Date {
        day: Some(now.day() as u8),
        month: Some(now.month() as u8),
        year: now.year(),
    }
}

impl TryFrom<Document> for Entry {
    type Error = HlpError;

    fn try_from(doc: Document) -> Result<Self, Self::Error> {
        let key = doc.find_key()?;
        let entry_type = doc.find_entry_type()?;

        let mut entry = Entry::new(&key, entry_type);

        entry.set_url(QualifiedUrl {
            value: doc.url.clone(),
            visit_date: Some(now()),
        });

        if let Ok(title) = doc.find_title() {
            entry.set_title(title)
        }

        Ok(entry)
    }
}

#[cfg(test)]
mod test {
    use hayagriva::{
        types::{EntryType, QualifiedUrl, Title},
        Entry,
    };
    use url::Url;

    use super::Document;

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
    fn document_find_key() {
        let doc = Document::parse(EXAMPLE_HTML, "http://example.com/").unwrap();

        let key = doc.find_key().unwrap();
        assert_eq!(key, "example-domain");
    }

    #[test]
    fn document_find_title() {
        let doc = Document::parse(EXAMPLE_HTML, "http://example.com/").unwrap();

        let title = doc.find_title().unwrap();
        assert_eq!(title.canonical.value, "Example Domain");
    }

    #[test]
    fn document_find_entry_type() {
        let doc = Document::parse(EXAMPLE_HTML, "http://example.com/").unwrap();

        let entry_type = doc.find_entry_type().unwrap();
        assert_eq!(entry_type, EntryType::Web)
    }

    #[test]
    fn entry_try_from_doc() {
        let doc = Document::parse(EXAMPLE_HTML, "https://example.com/").unwrap();

        let entry = Entry::try_from(doc).unwrap();

        let url = Url::parse("https://example.com").unwrap();
        let mut expected = Entry::new("example-domain", EntryType::Web);
        expected.set_title(Title::new("Example Domain"));
        expected.set_url(QualifiedUrl {
            visit_date: Some(super::now()),
            value: url,
        });

        assert_eq!(entry, expected);
    }
}
