use anyhow::Result;

#[cfg_attr(test, mockall::automock)]
pub(crate) trait Converter {
    fn convert<'a>(&self, pronunciation: &[&'a str]) -> Result<String>;
}
