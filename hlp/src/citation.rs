use hayagriva::{
    types::{Date, EntryType, Person, PersonRole, QualifiedUrl, Title},
    Entry,
};
use unic_langid::LanguageIdentifier;

use crate::DataReport;

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

impl From<DataReport> for CitationBuilder {
    fn from(report: DataReport) -> Self {
        todo!()
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

// impl TryFrom<Document> for Entry {
//     type Error = HlpError;

//     fn try_from(doc: Document) -> Result<Self, Self::Error> {
//         let key = doc.find_key()?;
//         let entry_type = doc.find_entry_type()?;

//         let mut entry = Entry::new(&key, entry_type);

//         entry.set_url(QualifiedUrl {
//             value: doc.url.clone(),
//             visit_date: Some(now()),
//         });

//         if let Ok(title) = doc.find_title() {
//             entry.set_title(title)
//         }

//         Ok(entry)
//     }
// }
