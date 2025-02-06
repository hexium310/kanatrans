use std::{fmt::Debug, future::Future, sync::Arc};

use axum::{
    extract::{Query, State},
    response::{IntoResponse, Response},
    Json,
};
use domain::processor::transliterator::Transliterator;
use serde::{Deserialize, Serialize};

use crate::error::ServiceError;

#[derive(Debug, Deserialize, Serialize)]
pub struct Katakana {
    pub pronunciation: String,
}

#[derive(Debug, Default)]
pub struct KatakanaService<Processor> {
    transliterator: Processor,
}

#[derive(Debug, Deserialize)]
pub struct Params {
    pub word: Option<String>,
    pub pronunciation: String,
}

pub trait KatakanaServiceInterface: Send + Sync + 'static {
    fn get(&self, params: &[&str]) -> impl Future<Output = Result<Katakana, ServiceError>> + Send;
}

impl<Processor> KatakanaServiceInterface for KatakanaService<Processor>
where
    Processor: Transliterator<Target: Into<String>> + Debug + Send + Sync + 'static,
{
    #[tracing::instrument(ret(level = tracing::Level::INFO), err)]
    async fn get(&self, pronunciation: &[&str]) -> Result<Katakana, ServiceError> {
        let katakana =
            self.transliterator
                .transliterate(pronunciation)
                .map_err(|e| ServiceError::ConvertKatakanaError {
                    pronunciation: pronunciation.iter().map(ToString::to_string).collect::<Vec<String>>(),
                    source: e,
                })?;
        let response = Katakana {
            pronunciation: katakana.into(),
        };

        Ok(response)
    }
}

impl<Processor> KatakanaService<Processor> {
    pub fn new(transliterator: Processor) -> Self {
        Self { transliterator }
    }
}

pub async fn handle<KatakanaService>(
    State(katakana_service): State<Arc<KatakanaService>>,
    Query(params): Query<Params>,
) -> Result<Response, ServiceError>
where
    KatakanaService: KatakanaServiceInterface,
{
    let katakana = katakana_service
        .get(&params.pronunciation.split_whitespace().collect::<Vec<_>>())
        .await?;

    Ok(Json(katakana).into_response())
}
