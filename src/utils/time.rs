use prost_types::Timestamp;
use serde::{self, Deserialize};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

#[allow(unused)]
pub fn serialize_offset_date_time<S>(
    date_time: &Option<OffsetDateTime>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match date_time {
        Some(dt) => {
            serializer.serialize_str(&dt.format(&Rfc3339).map_err(serde::ser::Error::custom)?)
        }
        None => serializer.serialize_none(),
    }
}

#[allow(unused)]
pub fn deserialize_offset_date_time<'de, D>(
    deserializer: D,
) -> Result<Option<OffsetDateTime>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) => Ok(Some(
            OffsetDateTime::parse(&s, &Rfc3339).map_err(serde::de::Error::custom)?,
        )),
        None => Ok(None),
    }
}

pub fn to_prost_timestamp(date_time: OffsetDateTime) -> Timestamp {
    Timestamp {
        seconds: date_time.unix_timestamp(),
        nanos: date_time.nanosecond() as i32,
    }
}

pub fn from_prost_timestamp(timestamp: Timestamp) -> Option<OffsetDateTime> {
    match OffsetDateTime::from_unix_timestamp(timestamp.seconds) {
        Ok(date_time) => Some(date_time),
        Err(_) => None,
    }
}
