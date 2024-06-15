use anyhow::Result;

pub(crate) trait Transliterator {
    type Target: Into<String>;

    fn transliterate(&self, pronunciation: &[&str]) -> Result<Self::Target>;
}
