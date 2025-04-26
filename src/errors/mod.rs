use axum::http::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sea_orm::DbErr),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Authentication error")]
    AuthenticationError,

    #[error("Authorization error")]
    AuthorizationError,

    #[error("Resource not found")]
    NotFound,

    #[error("Invalid request payload: {0}")]
    InvalidPayload(#[from] serde_json::Error),

    #[error("Media streaming error: {0}")]
    MediaStreamingError(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),
}

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, body) = match self {
            AppError::DatabaseError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error occurred".to_string(),
            ),
            AppError::ValidationError(message) => (StatusCode::BAD_REQUEST, message),
            AppError::AuthenticationError => (
                StatusCode::UNAUTHORIZED,
                "Authentication failed".to_string(),
            ),
            AppError::AuthorizationError => (
                StatusCode::FORBIDDEN,
                "Not authorized to perform this action".to_string(),
            ),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Resource not found".to_string()),
            AppError::InvalidPayload(e) => (
                StatusCode::BAD_REQUEST,
                format!("Invalid request payload: {}", e),
            ),
            AppError::MediaStreamingError(message) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Media streaming error: {}", message),
            ),
            AppError::InternalServerError(message) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal server error: {}", message),
            ),
        };

        (status, body).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
