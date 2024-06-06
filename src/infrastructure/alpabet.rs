use anyhow::Result;
use axum::{
    response::{IntoResponse, Response},
    Json,
};

use super::service::alpabet::AlpabetServiceInterface;
use crate::domain::{processor::transcriber::Transcriber, response::alpabet::Alpabet};

pub(crate) struct AlpabetService<Processor> {
    transcriber: Processor,
}

impl<Processor> AlpabetServiceInterface for AlpabetService<Processor>
where
    Processor: Transcriber + Send + Sync + 'static,
    <Processor as Transcriber>::Target: AsRef<str>,
{
    async fn get(&self, word: String) -> Result<Response> {
        let alpabet = self.transcriber.transcribe(&word)?;
        let response = Json(Alpabet {
            word,
            pronunciation: alpabet.as_ref().to_string(),
        })
        .into_response();

        Ok(response)
    }
}

impl<Processor> AlpabetService<Processor> {
    pub(crate) fn new(transcriber: Processor) -> Self {
        Self { transcriber }
    }
}
