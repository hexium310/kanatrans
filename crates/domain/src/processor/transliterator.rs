use anyhow::Result;

pub trait Transliterator {
    type Target: Into<String>;

    fn transliterate(&self, pronunciation: &[&str]) -> Result<Self::Target>;
}
