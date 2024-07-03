use axum::{response::{IntoResponse, Response}, Json};
use hyper::StatusCode;
use serde::Serialize;
use service::error::ServiceError;

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct ServerError(#[from] pub ServiceError);

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        #[derive(Debug, Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (status, message) = (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string());

        (status, Json(ErrorResponse { message })).into_response()
    }
}
