use anyhow::Result;
use axum::{
    response::{IntoResponse, Response},
    Json,
};

use super::service::katakana::KatakanaServiceInterface;
use crate::domain::{
    processor::transliterator::Transliterator,
    request::katakana::Params,
    response::katakana::Katakana,
};

pub(crate) struct KatakanaService<Processor> {
    transliterator: Processor,
}

impl<Processor> KatakanaServiceInterface for KatakanaService<Processor>
where
    Processor: Transliterator + Send + Sync + 'static,
    <Processor as Transliterator>::Target: AsRef<str>,
{
    async fn get(&self, Params { word, pronunciation }: Params) -> Result<Response> {
        let pronunciation = pronunciation.split_whitespace().collect::<Vec<_>>();
        let katakana = self.transliterator.transliterate(&pronunciation)?;

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
