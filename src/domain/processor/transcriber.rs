use anyhow::Result;

pub(crate) trait Transcriber {
    type Target: ToString;

    fn transcribe(&self, word: &str) -> Result<Self::Target>;
}
