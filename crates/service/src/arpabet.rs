use std::{fmt::Debug, future::Future, ops::Deref};

use domain::processor::transcriber::Transcriber;
use serde::{Deserialize, Serialize};

use crate::error::ServiceError;

#[derive(Debug, Deserialize, Serialize)]
pub struct Arpabet {
    pub word: String,
    pub pronunciation: Vec<String>,
}

#[derive(Debug)]
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
