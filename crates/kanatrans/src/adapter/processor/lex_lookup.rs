use anyhow::{bail, Result};
use flite::lexicon::lexicon;

use crate::{
    adapter::command::Executor,
    domain::{arpabet::Arpabet, processor::transcriber::Transcriber},
};

pub(crate) struct LexLookup<CommandExecutor> {
    executor: CommandExecutor,
}

pub(crate) struct LexLookupExecutor;

impl<CommandExecutor> Transcriber for LexLookup<CommandExecutor>
where
    CommandExecutor: Executor,
{
    type Target = Arpabet;

    fn transcribe(&self, word: &str) -> Result<Self::Target> {
        let output = self.executor.execute(word)?;
        Ok(Arpabet::from(output))
    }
}

impl Executor for LexLookupExecutor {
    fn execute(&self, word: &str) -> Result<Vec<String>> {
        match lexicon().lookup(&word.to_lowercase(), None) {
            Ok(lexs) if lexs.is_empty() => bail!("cannot convert to ARPAbet"),
            Ok(lexs) => Ok(lexs),
            Err(e) => bail!(e),
        }
    }
}

impl<CommandExecutor> LexLookup<CommandExecutor>
where
    CommandExecutor: Executor,
{
    pub(crate) fn new(executor: CommandExecutor) -> Self {
        Self { executor }
    }
}

impl LexLookupExecutor {
    pub(crate) fn new() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::LexLookup;
    use crate::{
        adapter::command::MockExecutor,
        domain::{arpabet::Arpabet, processor::transcriber::Transcriber},
    };

    #[test]
    fn transcribe() {
        let mut mock_executor = MockExecutor::new();
        mock_executor
            .expect_execute()
            .times(1)
            .withf(|x| x == "threshold")
            .returning(|_| Ok(["th", "r", "eh1", "sh", "ow1", "l", "d"].map(Into::into).to_vec()));

        let lex_lookup = LexLookup::new(mock_executor);
        let arpabet = lex_lookup.transcribe("threshold").unwrap();

        assert_eq!(
            arpabet,
            Arpabet(["th", "r", "eh1", "sh", "ow1", "l", "d"].map(Into::into).to_vec())
        )
    }
}
