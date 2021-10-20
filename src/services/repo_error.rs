#[allow(dead_code)]
pub enum Error {
    NotFound,
    UniqueViolation,
    BackendError,
}

impl Error {
    /// Map from diesel's error type to our own repo_error::Error type
    pub fn map_diesel_err(diesel_err: diesel::result::Error) -> Self {
        match diesel_err {
            diesel::result::Error::DatabaseError(kind, _) => match kind {
                diesel::result::DatabaseErrorKind::UniqueViolation => Self::UniqueViolation,

                _ => Self::BackendError,
            },
            _ => Self::BackendError,
        }
    }
}
