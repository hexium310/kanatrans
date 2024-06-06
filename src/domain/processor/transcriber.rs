use anyhow::Result;

pub(crate) trait Transcriber {
    type Target: AsRef<str>;

    fn transcribe(&self, word: &str) -> Result<Self::Target>;
}
