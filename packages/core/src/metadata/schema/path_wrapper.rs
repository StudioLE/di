use crate::prelude::*;
use sea_orm::sea_query::*;
use sea_orm::*;

/// A wrapper type for [`PathBuf`] that can be used as a `SeaORM` model field
///
/// - <https://www.sea-ql.org/SeaORM/docs/generate-entity/newtype/>
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PathWrapper(PathBuf);

// Convenience traits

impl Display for PathWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0.display())
    }
}

impl From<PathBuf> for PathWrapper {
    fn from(path: PathBuf) -> Self {
        PathWrapper(path)
    }
}

impl From<PathWrapper> for PathBuf {
    fn from(path: PathWrapper) -> Self {
        path.0
    }
}

impl FromStr for PathWrapper {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        PathBuf::from_str(s).map(PathWrapper)
    }
}

impl AsRef<PathBuf> for PathWrapper {
    fn as_ref(&self) -> &PathBuf {
        &self.0
    }
}

impl Deref for PathWrapper {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// SeaORM traits

impl From<PathWrapper> for Value {
    fn from(path: PathWrapper) -> Self {
        Value::String(Some(path.to_string()))
    }
}

impl TryGetable for PathWrapper {
    fn try_get_by<I: ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError> {
        let value: Option<String> = res.try_get_by(index).map_err(TryGetError::DbErr)?;
        match value {
            Some(s) => PathBuf::from_str(&s)
                .map(PathWrapper)
                .map_err(|e| TryGetError::DbErr(DbErr::Type(format!("Invalid URL: {e}")))),
            None => Err(TryGetError::Null(format!("{index:?}"))),
        }
    }
}

impl ValueType for PathWrapper {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::String(Some(s)) => PathBuf::from_str(&s)
                .map(PathWrapper)
                .map_err(|_| ValueTypeErr),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        "PathWrapper".to_owned()
    }

    fn array_type() -> ArrayType {
        ArrayType::String
    }

    fn column_type() -> ColumnType {
        ColumnType::String(StringLen::default())
    }
}

impl Nullable for PathWrapper {
    fn null() -> Value {
        Value::String(None)
    }
}
