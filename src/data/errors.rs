use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataError {
    #[error("Failed database query: {0}")]
    Query(#[from] sqlx::Error),

    #[error("Failed to query: {0}")]
    FailedQuery(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Failed to hash: {0}")]
    Bcrypt(#[from] bcrypt::BcryptError),

    #[error("Failed to convert from utf8: {0}")]
    Utf8Conversion(#[from] std::string::FromUtf8Error),
}