use std::{cell::OnceCell, str::FromStr};

use regex::Regex;

use crate::error::MetaError;

// let issn_pattern = ;
// let text = "The ISSN of this publication is 1234-567X.";
// if let Some(capture) = issn_pattern.find(text) {
//     let issn = capture.as_str();
//     println!("Found ISSN: {}", issn);
// }

#[derive(Debug, PartialEq, Eq)]
pub struct Issn {
    data: String,
}

impl FromStr for Issn {
    type Err = MetaError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        if is_issn(text) {
            Ok(Self {
                data: text.to_owned(),
            })
        } else {
            Err(MetaError::IssnParse)
        }
    }
}

impl From<Issn> for String {
    fn from(issn: Issn) -> Self {
        issn.data
    }
}

const ISSN_REGEX: OnceCell<Regex> = OnceCell::new();

fn is_issn(text: &str) -> bool {
    let cell = ISSN_REGEX;
    let regex = cell.get_or_init(|| Regex::new(r"^\d{4}-\d{3}[\dX]$").unwrap());

    regex.is_match(text)
}
