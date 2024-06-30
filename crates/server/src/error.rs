use axum::{response::{IntoResponse, Response}, Json};
use hyper::StatusCode;
use serde::Serialize;
use service::error::ServiceError;

#[derive(thiserror::Error, Debug)]
pub enum ServerError {
    #[error("getting arpabet failed")]
    ArpabetGetFailed(#[source] ServiceError),
    #[error("getting katakana failed")]
    KatakanaGetFailed(#[source] ServiceError),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        #[derive(Debug, Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (status, message) = match self {
            Self::ArpabetGetFailed(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            Self::KatakanaGetFailed(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
        };

        (status, Json(ErrorResponse { message })).into_response()
    }
}
