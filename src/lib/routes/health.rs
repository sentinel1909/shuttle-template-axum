// src/lib/handlers/health.rs

// dependencies
use axum::{http::StatusCode, response::IntoResponse};

// health check handler; returns 200 OK and an empty body
pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}
