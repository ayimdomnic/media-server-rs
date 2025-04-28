use thiserror::Error;

#[derive(Error, Debug)]
pub enum MediaError {
    #[error("Media file not found")]
    NotFound,
    #[error("Invalid media format")]
    InvalidFormat,
    #[error("Streaming error: {0}")]
    StreamingError(String),
    #[error("P2P connection failed")]
    P2PConnectionFailed,
}

impl From<MediaError> for crate::errors::AppError {
    fn from(error: MediaError) -> Self {
        match error {
            MediaError::NotFound => crate::errors::AppError::NotFound,
            _ => crate::errors::AppError::InternalServerError(error.to_string()),
        }
    }
}
