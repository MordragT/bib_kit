use std::str::FromStr;

use hayagriva::types::Person;

use crate::error::MetaError;

#[derive(Debug, PartialEq, Eq)]
pub struct Name {
    pub surname: String,
    pub given_name: Option<String>,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub alias: Option<String>,
}

impl FromStr for Name {
    type Err = MetaError;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        let name = human_name::Name::parse(name).ok_or(MetaError::NameParse)?;

        let surname = name.surname().to_owned();
        let given_name = match (name.given_name(), name.middle_name()) {
            (Some(forename), Some(middle_name)) => Some(format!("{forename} {middle_name}")),
            (Some(forename), None) => Some(forename.to_owned()),
            _ => None,
        };
        let prefix = name.honorific_prefix().map(|p| p.to_owned());
        let suffix = name.honorific_suffix().map(|s| s.to_owned());

        Ok(Self {
            surname,
            given_name,
            prefix,
            suffix,
            alias: None,
        })
    }
}

impl TryFrom<&str> for Name {
    type Error = MetaError;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        Self::from_str(name)
    }
}

impl TryFrom<String> for Name {
    type Error = MetaError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl From<Name> for Person {
    fn from(name: Name) -> Self {
        let Name {
            surname,
            given_name,
            prefix,
            suffix,
            alias,
        } = name;
        Person {
            name: surname,
            given_name,
            prefix,
            suffix,
            alias,
        }
    }
}
