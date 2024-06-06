use anyhow::Result;

pub(crate) trait Transliterator {
    type Target: AsRef<str>;

    fn transliterate(&self, word: &str) -> Result<Self::Target>;
}
