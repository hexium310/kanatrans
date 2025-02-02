use std::{collections::HashMap, convert::Infallible, fmt::Debug, future::Future, ops::Deref};

use domain::processor::transcriber::Transcriber;
use http::{Request, Response};
use serde::{Deserialize, Serialize};

use crate::{
    error::ServiceError,
    service::{Body, Handler},
};

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

impl<RequestBody, Processor> Handler<RequestBody> for ArpabetService<Processor>
where
    Processor: Transcriber<Target: Deref<Target = [String]>> + Debug + Send + Sync + 'static,
    RequestBody: Send,
{
    async fn handle(&self, req: Request<RequestBody>) -> Result<Response<Body>, Infallible> {
        let word = req
            .extensions()
            .get::<HashMap<String, String>>()
            .and_then(|params| params.get("word"))
            .expect("{word} path parameter must be required");

        match self.get(word.to_owned()).await {
            Ok(response) => Ok(Response::new(Body::from(
                serde_json::to_vec(&response).expect("Failed to serialize arpabet response body"),
            ))),
            Err(err) => Ok(err.into_response()),
        }
    }
}

impl<Processor> ArpabetService<Processor> {
    pub fn new(transcriber: Processor) -> Self {
        Self { transcriber }
    }
}
