use anyhow::Result;

pub(crate) trait Converter {
    fn convert(&self, pronunciation: &[&str]) -> Result<String>;
}
