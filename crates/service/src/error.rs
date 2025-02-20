use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use problem_details::ProblemDetails;

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("cannot parse {word:?} as ARPAbet, caused by: {source}")]
    ParseArpabetError {
        word: String,
        #[source]
        source: anyhow::Error,
    },
    #[error("cannot convert {pronunciation:?} to Katakana, caused by: {source}")]
    ConvertKatakanaError {
        pronunciation: Vec<String>,
        #[source]
        source: anyhow::Error,
    },
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        let details = ProblemDetails::from_status_code(StatusCode::INTERNAL_SERVER_ERROR).with_detail(self.to_string());

        details.into_response()
    }
}
