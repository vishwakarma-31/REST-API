use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Invalid sort column")]
    InvalidSortColumn,
    #[error("Invalid filter column")]
    InvalidFilterColumn,
    #[error("Invalid sort order")]
    InvalidSortOrder,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            AppError::DatabaseError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::InvalidSortColumn => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::InvalidFilterColumn => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::InvalidSortOrder => (StatusCode::BAD_REQUEST, self.to_string()),
        };
        (status, Json(json!({ "error": msg }))).into_response()
    }
}
