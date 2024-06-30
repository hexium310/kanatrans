use anyhow::Result;

#[cfg_attr(test, mockall::automock)]
pub trait Executor {
    fn execute(&self, word: &str) -> Result<Vec<String>>;
}
