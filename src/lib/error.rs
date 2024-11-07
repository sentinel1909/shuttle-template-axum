// src/lib/errors.rs

// dependencies
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

// struct type to represent an ApiError
#[derive(Debug, Clone, Error)]
pub enum ApiError {
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    Internal(String),
    #[error("{0}")]
    NotFound(String),
}

// implement the IntoResponse trait for the ApiError type
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            ApiError::BadRequest(_) => (StatusCode::BAD_REQUEST, "Bad Request"),
            ApiError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"),
            ApiError::NotFound(_) => (StatusCode::NOT_FOUND, "Not Found"),
        };

        (status, msg).into_response()
    }
}
