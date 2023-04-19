use crate::date::DateIso8601;

#[derive(Debug)]
pub struct OgpBook {
    /// Who wrote this book.
    pub authors: Vec<String>,
    /// The ISBN
    pub isbn: Option<String>,
    /// The date the book was released.
    pub release_date: Option<DateIso8601>,
    /// Tag words associated with this book.
    pub tags: Vec<String>,
}
