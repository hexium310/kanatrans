use http::{Response, StatusCode};

use crate::service::Body;

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

impl ServiceError {
    pub fn into_response(self) -> Response<Body> {
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(
                serde_json::to_vec(&serde_json::json!({
                    "message": self.to_string(),
                }))
                .expect("Failed to serialize error response body"),
            ))
            .expect("Failed to build error response")
    }
}
