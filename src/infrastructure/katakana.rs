use anyhow::Result;
use axum::{
    response::{IntoResponse, Response},
    Json,
};

use super::service::katakana::KatakanaServiceInterface;
use crate::domain::{processor::transliterator::Transliterator, response::katakana::Katakana};

pub(crate) struct KatakanaService<Processor> {
    transliterator: Processor,
}

impl<Processor> KatakanaServiceInterface for KatakanaService<Processor>
where
    Processor: Transliterator + Send + Sync + 'static,
    <Processor as Transliterator>::Target: AsRef<str>,
{
    async fn get(&self, word: String) -> Result<Response> {
        let katakana = self.transliterator.transliterate(&word)?;
        let response = Json(Katakana {
            word,
            pronunciation: katakana.as_ref().to_string(),
        })
        .into_response();

        Ok(response)
    }
}

impl<Processor> KatakanaService<Processor> {
    pub(crate) fn new(transliterator: Processor) -> Self {
        Self { transliterator }
    }
}
