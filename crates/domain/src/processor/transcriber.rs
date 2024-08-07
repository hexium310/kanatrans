use std::ops::Deref;

use anyhow::Result;

pub trait Transcriber {
    type Target: Deref<Target = [String]>;

    fn transcribe(&self, word: &str) -> Result<Self::Target>;
}
