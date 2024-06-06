use anyhow::Error;
use axum::response::{IntoResponse, Response};
use hyper::StatusCode;

pub(crate) struct AlpabetGetFailed(Error);
pub(crate) struct KatakanaGetFailed(Error);

impl<E> From<E> for AlpabetGetFailed
where
    E: Into<Error>,
{
    fn from(err: E) -> Self {
        AlpabetGetFailed(err.into())
    }
}

impl<E> From<E> for KatakanaGetFailed
where
    E: Into<Error>,
{
    fn from(err: E) -> Self {
        KatakanaGetFailed(err.into())
    }
}

impl IntoResponse for AlpabetGetFailed {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal Server Error: {}", self.0),
        )
            .into_response()
    }
}

impl IntoResponse for KatakanaGetFailed {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal Server Error: {}", self.0),
        )
            .into_response()
    }
}
