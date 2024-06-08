use anyhow::Result;

#[cfg_attr(test, mockall::automock)]
pub(crate) trait Executor {
    fn execute(&self, word: &str) -> Result<String>;
}
