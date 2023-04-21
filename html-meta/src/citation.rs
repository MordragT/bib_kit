#![allow(unused_variables)]

use crate::{
    dom::Dom,
    error::{MetaError, MetaResult},
    meta::{
        generic::GenericMetadata,
        ogp::{OgpArticle, OgpAudio, OgpBook, OgpImage, OgpMetadata, OgpVideo},
        value::{into_qualified, DateIso8601, Name, Title},
    },
    priority::PriorityData,
    query::HtmlQueryReport,
};
use hayagriva::{io::to_yaml_str, types::EntryType, Entry};
use isbn2::Isbn;
use unic_langid::LanguageIdentifier;
use url::Url;
use wasm_bindgen::prelude::wasm_bindgen;

const PLACEHOLDER: &'static str = "placeholder";

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq)]
pub struct Citation {
    entry: Entry,
}

#[wasm_bindgen]
impl Citation {
    #[wasm_bindgen(constructor)]
    pub fn new(dom: Dom) -> MetaResult<Citation> {
        let citation = CitationBuilder::new(dom)
            .with_html_query_report()?
            .with_generic_metadata()?
            .with_ogp_metadata()?
            .build();

        Ok(citation)
    }

    pub fn to_yaml_str(&self) -> MetaResult<String> {
        to_yaml_str([&self.entry]).ok_or(MetaError::YamlParse)
    }
}

impl From<Entry> for Citation {
    fn from(entry: Entry) -> Self {
        Self { entry }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CitationBuilder {
    entry_type: EntryType,
    dom: Dom,
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

impl CitationBuilder {
    pub fn new(dom: Dom) -> Self {
        let url = PriorityData {
            first: Some(dom.url().clone()),
            second: None,
            third: None,
        };

        Self {
            dom,
            entry_type: EntryType::Web,
            title: Default::default(),
            authors: Default::default(),
            date: Default::default(),
            editors: Default::default(),
            publisher: Default::default(),
            location: Default::default(),
            organization: Default::default(),
            url,
            serial_number: Default::default(),
            isbn: Default::default(),
            language: Default::default(),
            note: Default::default(),
        }
    }

    pub fn with_html_query_report(mut self) -> MetaResult<Self> {
        let HtmlQueryReport { title } = self.dom.html_query_report()?;

        if let Some(title) = title {
            self.title.third = Some(title);
        }

        Ok(self)
    }

    pub fn with_generic_metadata(mut self) -> MetaResult<Self> {
        let GenericMetadata {
            title,
            content_type,
            description,
            keywords,
            language,
            author,
        } = self.dom.generic_metadata()?;

        if let Some(title) = title {
            self.title.second = Some(title);
        }

        if let Some(language) = language {
            self.language.second = Some(language);
        }

        if let Some(author) = author {
            self.authors.second = Some(vec![author]);
        }

        Ok(self)
    }

    pub fn with_ogp_metadata(mut self) -> MetaResult<Self> {
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
        } = self.dom.ogp_metadata()?;

        if let Some(title) = title {
            self.title.first = Some(title);
        }

        if let Some(url) = url {
            self.url.first = Some(url);
        }

        if let Some(language) = locale {
            self.language.first = Some(language);
        }

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

        Ok(self)
    }

    pub fn build(self) -> Citation {
        Entry::from(self).into()
    }
}

impl From<CitationBuilder> for Entry {
    fn from(citation: CitationBuilder) -> Self {
        let CitationBuilder {
            dom: _,
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
