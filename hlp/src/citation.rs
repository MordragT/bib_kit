use hayagriva::{
    types::{Date, EntryType, Person, PersonRole, QualifiedUrl, Title},
    Entry,
};
use unic_langid::LanguageIdentifier;

use crate::{date::DateIso8601, DataReport};

pub const PLACEHOLDER: &'static str = "placeholder";

pub struct CitationBuilder {
    key: String,
    entry_type: EntryType,
    title: Option<Title>,
    authors: Vec<Person>,
    date: Option<Date>,
    editors: Vec<Person>,
    affiliated_persons: Vec<(Vec<Person>, PersonRole)>,
    publisher: Option<String>,
    location: Option<String>,
    organization: Option<String>,
    url: Option<QualifiedUrl>,
    serial_number: Option<String>,
    isbn: Option<String>,
    language: Option<LanguageIdentifier>,
    note: Option<String>,
}

impl CitationBuilder {
    pub fn new(report: DataReport) -> Self {
        Self::from(report)
    }

    pub fn build(self) -> Entry {
        Entry::from(self)
    }
}

// TODO transform data earlier at the relevant sections in ogp_meta generic_meta etc.
impl From<DataReport> for CitationBuilder {
    fn from(report: DataReport) -> Self {
        let DataReport {
            key,
            kind,
            title,
            note,
            language,
            authors,
            url,
        } = report;

        let key = match key {
            Some(key) => key,
            None => title
                .as_ref()
                .map(|title| calculate_key(title))
                .unwrap_or(PLACEHOLDER.to_owned()),
        };

        let entry_type = EntryType::Web;

        let title = title.map(|title| Title {
            canonical: title.into(),
            shorthand: None,
            translated: None,
        });

        let authors = match authors {
            Some(authors) => authors
                .into_iter()
                .map(|author| Person {
                    name: author,
                    given_name: None,
                    prefix: None,
                    suffix: None,
                    alias: None,
                })
                .collect(),
            None => Vec::new(),
        };

        let url = url.map(|url| QualifiedUrl {
            value: url,
            visit_date: Some(DateIso8601::now().into()),
        });

        // let language = language.map(|lang| LanguageIdentifier::from_str(&lang))

        Self {
            key,
            entry_type,
            title,
            authors,
            date: None,
            editors: Vec::new(),
            affiliated_persons: Vec::new(),
            publisher: None,
            location: None,
            organization: None,
            url,
            serial_number: None,
            isbn: None,
            language: None,
            note,
        }
    }
}

impl From<CitationBuilder> for Entry {
    fn from(citation: CitationBuilder) -> Self {
        let CitationBuilder {
            key,
            entry_type,
            title,
            authors,
            date,
            editors,
            affiliated_persons,
            publisher,
            location,
            organization,
            url,
            serial_number,
            isbn,
            language,
            note,
        } = citation;

        let mut entry = Entry::new(&key, entry_type);

        if let Some(title) = title {
            entry.set_title(title);
        }

        entry.set_authors(authors);

        if let Some(date) = date {
            entry.set_date(date);
        }

        entry.set_editors(editors);
        entry.set_affiliated_persons(affiliated_persons);

        if let Some(publisher) = publisher {
            entry.set_publisher(publisher.into());
        }
        if let Some(location) = location {
            entry.set_location(location.into());
        }

        if let Some(orga) = organization {
            entry.set_organization(orga);
        }

        if let Some(url) = url {
            entry.set_url(url);
        }

        if let Some(serial_number) = serial_number {
            entry.set_serial_number(serial_number);
        }
        if let Some(isbn) = isbn {
            entry.set_isbn(isbn);
        }

        if let Some(language) = language {
            entry.set_language(language);
        }

        if let Some(note) = note {
            entry.set_note(note);
        }

        entry
    }
}

fn calculate_key(title: &str) -> String {
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
