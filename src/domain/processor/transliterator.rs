use anyhow::Result;

pub(crate) trait Transliterator {
    type Target: AsRef<str>;

    fn transliterate(&self, pronunciation: &[&str]) -> Result<Self::Target>;
}
