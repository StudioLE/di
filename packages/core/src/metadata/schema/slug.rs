use crate::prelude::*;
use sea_orm::DeriveValueType;

#[derive(Clone, Debug, Deserialize, DeriveValueType, Eq, Hash, PartialEq, Serialize)]
pub struct Slug(String);

impl Slug {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for Slug {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Slug {
    type Err = SlugError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.is_empty() {
            return Err(SlugError::Empty);
        }
        if value.starts_with('-') {
            return Err(SlugError::StartsWithDash);
        }
        if value.ends_with('-') {
            return Err(SlugError::EndsWithDash);
        }
        if !value.chars().all(|c| c.is_ascii_lowercase() || c == '-') {
            return Err(SlugError::AllowedCharacters);
        }
        Ok(Self(value.to_owned()))
    }
}

#[derive(Debug, Error)]
pub enum SlugError {
    #[error("Must not be empty")]
    Empty,
    #[error("Must not start with a dash")]
    StartsWithDash,
    #[error("Must not end with a dash")]
    EndsWithDash,
    #[error("Must be only lowercase characters and dash")]
    AllowedCharacters,
}
