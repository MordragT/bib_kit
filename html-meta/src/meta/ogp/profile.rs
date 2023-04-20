use super::OgpMetadata;
use crate::{
    error::{MetaError, MetaResult},
    meta::{
        item::{MetadataItem, MetadataItemExt, MetadataItemExtractor, MetadataItemTarget},
        value::Name,
    },
};
use scraper::{ElementRef, Selector};

pub enum OgpProfileItem {
    Check,
    FirstName,
    LastName,
    Username,
    Gender,
}

impl MetadataItem for OgpProfileItem {
    fn selector(&self) -> MetaResult<Selector> {
        let selector = match self {
            Self::Check => Selector::parse("meta[property=\"og:type\"][content=\"profile\"]")?,
            Self::FirstName => Selector::parse("meta[property=\"og:profile:first_name\"]")?,
            Self::LastName => Selector::parse("meta[property=\"og:profile:last_name\"]")?,
            Self::Username => Selector::parse("meta[property=\"og:profile:username\"]")?,
            Self::Gender => Selector::parse("meta[property=\"og:profile:gender\"]")?,
        };

        Ok(selector)
    }

    fn target(&self) -> MetadataItemTarget {
        match self {
            Self::Check => MetadataItemTarget::None,
            _ => MetadataItemTarget::Attr("content"),
        }
    }
}

impl MetadataItemExt for OgpProfileItem {}

impl MetadataItemExtractor<OgpMetadata> for OgpProfileItem {
    fn extract_item_to(&mut self, el: &ElementRef, target: &mut OgpMetadata) -> MetaResult<()> {
        match self {
            Self::Check => target.profile.is_profile = true,
            Self::FirstName => {
                let first_name = self.extract(el)?;
                target.profile.first_name = Some(first_name);
            }
            Self::LastName => {
                let last_name = self.extract(el)?;
                target.profile.last_name = Some(last_name);
            }
            Self::Username => {
                let username = self.extract(el)?;
                target.profile.username = Some(username);
            }
            Self::Gender => {
                let gender = self.extract(el)?;
                target.profile.gender = Some(gender);
            }
        }

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct OgpProfile {
    pub is_profile: bool,
    /// A name normally given to an individual by a parent or self-chosen.
    pub first_name: Option<String>,
    ///  A name inherited from a family or marriage and by which the individual is commonly known.
    pub last_name: Option<String>,
    /// A short unique string to identify them.
    pub username: Option<String>,
    /// Their gender.
    pub gender: Option<String>,
}

impl TryFrom<OgpProfile> for Name {
    type Error = MetaError;

    fn try_from(profile: OgpProfile) -> Result<Self, Self::Error> {
        let OgpProfile {
            is_profile: _,
            first_name,
            last_name,
            username,
            gender: _,
        } = profile;
        if let Some(surname) = last_name {
            Ok(Name {
                surname,
                given_name: first_name,
                alias: username,
                prefix: None,
                suffix: None,
            })
        } else {
            Err(MetaError::MissingMetadata)
        }
    }
}
