#[allow(dead_code)]
#[allow(clippy::enum_variant_names)]
pub enum Error {
    NotFound,
    UniqueViolation,
    BackendError,
}

impl Error {
    /// Map from diesel's error type to our own repo_error::Error type
    pub fn map_diesel_err(diesel_err: diesel::result::Error) -> Self {
        match diesel_err {
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            ) => Self::UniqueViolation,
            _ => Self::BackendError,
        }
    }
}
