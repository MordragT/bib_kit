use std::{convert::Infallible, str::FromStr};

use deunicode::deunicode;

#[derive(Debug, PartialEq, Eq)]
pub struct Title {
    pub canonical: String,
}

impl Title {
    pub fn new(canonical: &str) -> Self {
        Self {
            canonical: deunicode(canonical),
        }
    }
}

impl FromStr for Title {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            canonical: deunicode(s),
        })
    }
}

impl From<String> for Title {
    fn from(canonical: String) -> Self {
        let canonical = deunicode(&canonical);
        Self { canonical }
    }
}

impl From<Title> for hayagriva::types::Title {
    fn from(title: Title) -> Self {
        Self::new(title.canonical)
    }
}
