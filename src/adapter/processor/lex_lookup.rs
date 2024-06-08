use anyhow::Result;

use crate::domain::{arpabet::Arpabet, processor::transcriber::Transcriber};

pub(crate) struct LexLookup;

impl Transcriber for LexLookup {
    type Target = Alpabet;

    fn transcribe(&self, word: &str) -> Result<Self::Target> {
        todo!()
    }
}

impl LexLookup {
    pub(crate) fn new() -> Self {
        Self
    }
}
