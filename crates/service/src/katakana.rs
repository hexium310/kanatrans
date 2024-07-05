use std::{fmt::Debug, future::Future};

use domain::processor::transliterator::Transliterator;
use serde::{Deserialize, Serialize};

use crate::error::ServiceError;

#[derive(Debug, Deserialize, Serialize)]
pub struct Katakana {
    pub pronunciation: String,
}

#[derive(Debug)]
pub struct KatakanaService<Processor> {
    transliterator: Processor,
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
