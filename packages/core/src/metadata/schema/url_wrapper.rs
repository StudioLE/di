use crate::prelude::*;
use sea_orm::sea_query::*;
use sea_orm::*;
use url::ParseError;

/// A wrapper type for URL that can be used as a `SeaORM` model field
///
/// - <https://www.sea-ql.org/SeaORM/docs/generate-entity/newtype/>
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct UrlWrapper(Url);

// Convenience traits

impl Display for UrlWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl From<Url> for UrlWrapper {
    fn from(url: Url) -> Self {
        UrlWrapper(url)
    }
}

impl From<UrlWrapper> for Url {
    fn from(url: UrlWrapper) -> Self {
        url.0
    }
}

impl FromStr for UrlWrapper {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Url::parse(s).map(UrlWrapper)
    }
}

impl AsRef<Url> for UrlWrapper {
    fn as_ref(&self) -> &Url {
        &self.0
    }
}

impl Deref for UrlWrapper {
    type Target = Url;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// SeaORM traits

impl From<UrlWrapper> for Value {
    fn from(url: UrlWrapper) -> Self {
        Value::String(Some(url.0.to_string()))
    }
}

impl TryGetable for UrlWrapper {
    fn try_get_by<I: ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError> {
        let value: Option<String> = res.try_get_by(index).map_err(TryGetError::DbErr)?;
        match value {
            Some(s) => Url::from_str(&s)
                .map(UrlWrapper)
                .map_err(|e| TryGetError::DbErr(DbErr::Type(format!("Invalid URL: {e}")))),
            None => Err(TryGetError::Null(format!("{index:?}"))),
        }
    }
}

impl ValueType for UrlWrapper {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::String(Some(s)) => Url::from_str(&s).map(UrlWrapper).map_err(|_| ValueTypeErr),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        "UrlWrapper".to_owned()
    }

    fn array_type() -> ArrayType {
        ArrayType::String
    }

    fn column_type() -> ColumnType {
        ColumnType::String(StringLen::N(2048))
    }
}

impl Nullable for UrlWrapper {
    fn null() -> Value {
        Value::String(None)
    }
}
