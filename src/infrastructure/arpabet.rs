use anyhow::Result;
use axum::{
    response::{IntoResponse, Response},
    Json,
};

use super::service::arpabet::ArpabetServiceInterface;
use crate::domain::{processor::transcriber::Transcriber, response::arpabet::Arpabet};

pub(crate) struct ArpabetService<Processor> {
    transcriber: Processor,
}

impl<Processor> ArpabetServiceInterface for ArpabetService<Processor>
where
    Processor: Transcriber + Send + Sync + 'static,
    <Processor as Transcriber>::Target: ToString,
{
    async fn get(&self, word: String) -> Result<Response> {
        let arpabet = self.transcriber.transcribe(&word)?;
        let response = Json(Arpabet {
            word,
            pronunciation: arpabet.to_string(),
        })
        .into_response();

        Ok(response)
    }
}

impl<Processor> ArpabetService<Processor> {
    pub(crate) fn new(transcriber: Processor) -> Self {
        Self { transcriber }
    }
}
