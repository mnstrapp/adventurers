use sqlx::{Error, postgres::PgRow};

pub trait DatabaseResource {
    fn from_row(row: &PgRow) -> Result<Self, Error>
    where
        Self: Sized;
    fn has_id() -> bool;
    fn is_archivable() -> bool;
    fn is_updatable() -> bool;
    fn is_creatable() -> bool;
    fn is_expirable() -> bool;
    fn is_verifiable() -> bool;
}
