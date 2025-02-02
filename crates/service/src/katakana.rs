use std::{convert::Infallible, fmt::Debug, future::Future};

use domain::processor::transliterator::Transliterator;
use http::{Request, Response};
use serde::{Deserialize, Serialize};

use crate::{error::ServiceError, Body, Handler};

#[derive(Debug, Deserialize, Serialize)]
pub struct Katakana {
    pub pronunciation: String,
}

#[derive(Debug, Default)]
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

impl<RequestBody, Processor> Handler<RequestBody> for KatakanaService<Processor>
where
    Processor: Transliterator<Target: Into<String>> + Debug + Send + Sync + 'static,
    RequestBody: Send,
{
    async fn handle(&self, req: Request<RequestBody>) -> Result<Response<Body>, Infallible> {
        #[derive(Debug, Deserialize)]
        struct Query {
            pronunciation: String,
        }

        let query = req.uri().query().unwrap_or_default();
        let query = serde_qs::from_str::<Query>(query).unwrap();

        match self.get(&query.pronunciation.split(" ").collect::<Vec<_>>()).await {
            Ok(response) => Ok(Response::new(Body::from(
                serde_json::to_vec(&response).expect("Failed to serialize katakana response body"),
            ))),
            Err(err) => Ok(err.into_response()),
        }
    }
}

impl<Processor> KatakanaService<Processor> {
    pub fn new(transliterator: Processor) -> Self {
        Self { transliterator }
    }
}
