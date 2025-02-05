use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

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
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "message": self.to_string() })),
        )
            .into_response()
    }
}
