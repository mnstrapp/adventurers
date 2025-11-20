use sqlx::postgres::PgArgumentBuffer;
use sqlx::{Encode, Postgres, Type, encode::IsNull, error::BoxDynError};
use std::fmt::{self, Display};
use std::iter::FromIterator;
use time::OffsetDateTime;

#[derive(Debug, Clone)]
pub enum DatabaseValue {
    None,
    #[allow(dead_code)]
    Str(&'static str),
    String(String),
    Text(String),
    Int(String),
    Int32(i32),
    Int64(String),
    Float(String),
    Boolean(String),
    DateTime(String),
}

impl Display for DatabaseValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<'q> Encode<'q, Postgres> for DatabaseValue {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        match self {
            DatabaseValue::None => Ok(IsNull::Yes),
            DatabaseValue::Str(s) => Encode::<Postgres>::encode_by_ref(s, buf),
            DatabaseValue::String(s) => Encode::<Postgres>::encode_by_ref(s, buf),
            DatabaseValue::Text(s) => Encode::<Postgres>::encode_by_ref(s, buf),
            DatabaseValue::Int(i) => Encode::<Postgres>::encode_by_ref(i, buf),
            DatabaseValue::Int32(i) => Encode::<Postgres>::encode_by_ref(i, buf),
            DatabaseValue::Int64(i) => Encode::<Postgres>::encode_by_ref(i, buf),
            DatabaseValue::Float(f) => Encode::<Postgres>::encode_by_ref(f, buf),
            DatabaseValue::Boolean(b) => Encode::<Postgres>::encode_by_ref(b, buf),
            DatabaseValue::DateTime(dt) => Encode::<Postgres>::encode_by_ref(dt, buf),
        }
    }
}

impl Type<Postgres> for DatabaseValue {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("text")
    }

    fn compatible(ty: &sqlx::postgres::PgTypeInfo) -> bool {
        let text_oids = [25, 1043, 1042, 19, 1042];
        ty.oid()
            .map(|oid| text_oids.contains(&oid.0))
            .unwrap_or(false)
    }
}

impl<'a> FromIterator<&'a str> for DatabaseValue {
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        DatabaseValue::String(iter.into_iter().collect::<String>())
    }
}

impl FromIterator<String> for DatabaseValue {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        DatabaseValue::String(iter.into_iter().collect())
    }
}

impl<'a> FromIterator<&'a String> for DatabaseValue {
    fn from_iter<I: IntoIterator<Item = &'a String>>(iter: I) -> Self {
        DatabaseValue::String(iter.into_iter().cloned().collect())
    }
}

impl FromIterator<bool> for DatabaseValue {
    fn from_iter<I: IntoIterator<Item = bool>>(iter: I) -> Self {
        DatabaseValue::Boolean(iter.into_iter().map(|b| b.to_string()).collect())
    }
}

impl FromIterator<OffsetDateTime> for DatabaseValue {
    fn from_iter<I: IntoIterator<Item = OffsetDateTime>>(iter: I) -> Self {
        DatabaseValue::DateTime(iter.into_iter().map(|dt| dt.to_string()).collect())
    }
}

impl FromIterator<i32> for DatabaseValue {
    fn from_iter<I: IntoIterator<Item = i32>>(iter: I) -> Self {
        DatabaseValue::Int(iter.into_iter().map(|i| i.to_string()).collect())
    }
}

impl FromIterator<i64> for DatabaseValue {
    fn from_iter<I: IntoIterator<Item = i64>>(iter: I) -> Self {
        DatabaseValue::Int64(iter.into_iter().map(|i| i.to_string()).collect())
    }
}

impl FromIterator<f64> for DatabaseValue {
    fn from_iter<I: IntoIterator<Item = f64>>(iter: I) -> Self {
        DatabaseValue::Float(iter.into_iter().map(|f| f.to_string()).collect())
    }
}

impl From<&str> for DatabaseValue {
    fn from(s: &str) -> Self {
        DatabaseValue::String(s.to_string())
    }
}

impl From<String> for DatabaseValue {
    fn from(s: String) -> Self {
        DatabaseValue::String(s)
    }
}

impl From<&'_ String> for DatabaseValue {
    fn from(s: &'_ String) -> Self {
        DatabaseValue::String(s.clone())
    }
}

impl From<bool> for DatabaseValue {
    fn from(b: bool) -> Self {
        DatabaseValue::Boolean(b.to_string())
    }
}

impl From<OffsetDateTime> for DatabaseValue {
    fn from(dt: OffsetDateTime) -> Self {
        DatabaseValue::DateTime(dt.to_string())
    }
}

impl From<i32> for DatabaseValue {
    fn from(i: i32) -> Self {
        DatabaseValue::Int(i.to_string())
    }
}

impl From<i64> for DatabaseValue {
    fn from(i: i64) -> Self {
        DatabaseValue::Int64(i.to_string())
    }
}

impl From<f64> for DatabaseValue {
    fn from(f: f64) -> Self {
        DatabaseValue::Float(f.to_string())
    }
}

impl<T: Into<DatabaseValue>> From<Option<T>> for DatabaseValue {
    fn from(option: Option<T>) -> Self {
        if option.is_none() {
            return DatabaseValue::None;
        }
        option.unwrap().into()
    }
}
