use anyhow::Error;
use axum::response::{IntoResponse, Response};
use hyper::StatusCode;

pub(crate) struct ArpabetGetFailed(Error);
pub(crate) struct KatakanaGetFailed(Error);

impl<E> From<E> for ArpabetGetFailed
where
    E: Into<Error>,
{
    fn from(err: E) -> Self {
        ArpabetGetFailed(err.into())
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

impl IntoResponse for ArpabetGetFailed {
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
