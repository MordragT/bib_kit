#![allow(unused_variables)]

use crate::{
    meta::{
        generic::GenericMetadata,
        ogp::{OgpArticle, OgpAudio, OgpBook, OgpImage, OgpMetadata, OgpVideo},
        value::{into_qualified, DateIso8601, Name, Title},
    },
    query::HtmlQueryReport,
    PriorityData,
};
use hayagriva::{types::EntryType, Entry};
use isbn2::Isbn;
use unic_langid::LanguageIdentifier;
use url::Url;

pub const PLACEHOLDER: &'static str = "placeholder";

#[derive(Debug)]
pub struct CitationBuilder {
    entry_type: EntryType,
    title: PriorityData<Title>,
    authors: PriorityData<Vec<Name>>,
    date: PriorityData<DateIso8601>,
    editors: PriorityData<Vec<Name>>,
    // affiliated_persons: Vec<(Vec<Person>, PersonRole)>,
    publisher: PriorityData<String>,
    location: PriorityData<String>,
    organization: PriorityData<String>,
    url: PriorityData<Url>,
    serial_number: PriorityData<String>,
    isbn: PriorityData<Isbn>,
    language: PriorityData<LanguageIdentifier>,
    note: PriorityData<String>,
}

impl Default for CitationBuilder {
    fn default() -> Self {
        Self {
            entry_type: EntryType::Web,
            title: Default::default(),
            authors: Default::default(),
            date: Default::default(),
            editors: Default::default(),
            publisher: Default::default(),
            location: Default::default(),
            organization: Default::default(),
            url: Default::default(),
            serial_number: Default::default(),
            isbn: Default::default(),
            language: Default::default(),
            note: Default::default(),
        }
    }
}

impl CitationBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_url(mut self, url: Url) -> Self {
        self.url.first = Some(url);
        self
    }

    pub fn with_html_query_report(mut self, html_query_report: HtmlQueryReport) -> Self {
        let HtmlQueryReport { title } = html_query_report;

        self.title.third = title;

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
        self.language.second = language;
        self.authors.second = author.map(|a| vec![a]);

        self
    }

    pub fn with_ogp_metadata(mut self, ogp_metadata: OgpMetadata) -> Self {
        let OgpMetadata {
            title,
            kind,
            image,
            url,
            audio,
            description,
            locale,
            alternate_locales,
            site_name,
            video,
            article,
            book,
            profile,
        } = ogp_metadata;

        self.title.first = title;
        self.url.first = url;
        self.language.first = locale;

        let OgpImage {
            url,
            secure_url,
            mime,
            width,
            height,
            alt,
        } = image;

        let OgpAudio {
            url,
            secure_url,
            mime,
        } = audio;

        let OgpVideo {
            url,
            secure_url,
            mime,
            width,
            height,
            alt,
        } = video;

        if let Some(url) = url {
            self.entry_type = EntryType::Video;
            self.url.first = Some(url);
        }

        let OgpArticle {
            is_article,
            published_time,
            modified_time,
            expiration_time,
            authors,
            section,
            tags,
        } = article;

        if is_article {
            self.entry_type = EntryType::Article;
        }

        if authors.len() > 0 {
            self.entry_type = EntryType::Article;
            self.authors.first = Some(authors);
        }

        let OgpBook {
            is_book,
            authors,
            isbn,
            release_date,
            tags,
        } = book;

        if is_book {
            self.entry_type = EntryType::Book;
        }

        if authors.len() > 0 {
            self.entry_type = EntryType::Book;
            self.authors.first = Some(authors);
        }

        if let Some(isbn) = isbn {
            self.entry_type = EntryType::Book;
            self.isbn.first = Some(isbn);
        }

        self
    }

    pub fn build(self) -> Entry {
        Entry::from(self)
    }
}

impl From<CitationBuilder> for Entry {
    fn from(citation: CitationBuilder) -> Self {
        let CitationBuilder {
            entry_type,
            title,
            authors,
            date,
            editors,
            // affiliated_persons,
            publisher,
            location,
            organization,
            url,
            serial_number,
            isbn,
            language,
            note,
        } = citation;

        let highest_title = title.highest();

        let key = highest_title
            .as_ref()
            .map(calculate_key)
            .unwrap_or(PLACEHOLDER.to_owned());

        let mut entry = Entry::new(&key, entry_type);

        if let Some(title) = highest_title {
            entry.set_title(title.into());
        }

        if let Some(authors) = authors.highest() {
            let item = authors.into_iter().map(Into::into).collect();
            entry.set_authors(item);
        }

        if let Some(date) = date.highest() {
            entry.set_date(date.into());
        }

        if let Some(editors) = editors.highest() {
            let item = editors.into_iter().map(Into::into).collect();
            entry.set_editors(item);
        }

        if let Some(publisher) = publisher.highest() {
            entry.set_publisher(publisher.into());
        }
        if let Some(location) = location.highest() {
            entry.set_location(location.into());
        }

        if let Some(orga) = organization.highest() {
            entry.set_organization(orga);
        }

        if let Some(url) = url.highest() {
            entry.set_url(into_qualified(url));
        }

        if let Some(serial_number) = serial_number.highest() {
            entry.set_serial_number(serial_number);
        }
        if let Some(isbn) = isbn.highest() {
            entry.set_isbn(isbn.to_string());
        }

        if let Some(language) = language.highest() {
            entry.set_language(language);
        }

        if let Some(note) = note.highest() {
            entry.set_note(note);
        }

        entry
    }
}

// TODO remove special characters
fn calculate_key(title: &Title) -> String {
    // TODO add author to key if found
    let key = title
        .canonical
        .to_ascii_lowercase()
        .replace(|c: char| !c.is_ascii(), "");
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
