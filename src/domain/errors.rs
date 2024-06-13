use anyhow::Error;
use axum::{response::{IntoResponse, Response}, Json};
use hyper::StatusCode;
use serde::Serialize;

pub(crate) enum ApiError {
    ArpabetGetFailed(Error),
    KatakanaGetFailed(Error)
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        #[derive(Debug, Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (status, message) = match self {
            ApiError::ArpabetGetFailed(err) => {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    err.to_string(),
                )
            },
            ApiError::KatakanaGetFailed(err) => {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    err.to_string(),
                )
            },
        };

        (status, Json(ErrorResponse { message })).into_response()
    }
}
