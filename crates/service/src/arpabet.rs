use std::{fmt::Debug, future::Future, ops::Deref, sync::Arc};

use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
    Json,
};
use domain::processor::transcriber::Transcriber;
use serde::{Deserialize, Serialize};

use crate::error::ServiceError;

#[derive(Debug, Deserialize, Serialize)]
pub struct Arpabet {
    pub word: String,
    pub pronunciation: Vec<String>,
}

#[derive(Debug, Default)]
pub struct ArpabetService<Processor> {
    transcriber: Processor,
}

pub trait ArpabetServiceInterface: Send + Sync + 'static {
    fn get(&self, word: String) -> impl Future<Output = Result<Arpabet, ServiceError>> + Send;
}

impl<Processor> ArpabetServiceInterface for ArpabetService<Processor>
where
    Processor: Transcriber<Target: Deref<Target = [String]>> + Debug + Send + Sync + 'static,
{
    #[tracing::instrument(ret(level = tracing::Level::INFO), err)]
    async fn get(&self, word: String) -> Result<Arpabet, ServiceError> {
        let arpabet = self
            .transcriber
            .transcribe(&word)
            .map_err(|e| ServiceError::ParseArpabetError {
                word: word.clone(),
                source: e,
            })?;
        let response = Arpabet {
            word,
            pronunciation: arpabet.to_vec(),
        };

        Ok(response)
    }
}

impl<Processor> ArpabetService<Processor> {
    pub fn new(transcriber: Processor) -> Self {
        Self { transcriber }
    }
}

pub async fn handle<ArpabetService>(
    State(arpabet_service): State<Arc<ArpabetService>>,
    Path(word): Path<String>,
) -> Result<Response, ServiceError>
where
    ArpabetService: ArpabetServiceInterface,
{
    let arpabet = arpabet_service.get(word).await?;

    Ok(Json(arpabet).into_response())
}
