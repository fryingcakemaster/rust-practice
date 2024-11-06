use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[allow(unused)]
pub enum ApiError {
    BadRequest,
    InternalServerError,
    DbError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            Self::BadRequest => (StatusCode::BAD_REQUEST).into_response(),
            Self::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
            Self::DbError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response(),
        }
    }
}
