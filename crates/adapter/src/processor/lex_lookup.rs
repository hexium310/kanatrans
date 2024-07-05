use anyhow::{anyhow, bail, Result};
use domain::{arpabet::Arpabet, processor::transcriber::Transcriber};
use flite::lexicon::lexicon;

use crate::{command::Executor, error::ProcessorError};

#[derive(Debug)]
pub struct LexLookup<CommandExecutor> {
    executor: CommandExecutor,
}

#[derive(Debug)]
pub struct LexLookupExecutor;

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
            Ok(lexs) if lexs.is_empty() => bail!(ProcessorError::NoPhonemes { word: word.to_string() }),
            Ok(lexs) => Ok(lexs),
            Err(e) => bail!(ProcessorError::InvalidCharacter {
                word: word.to_string(),
                source: anyhow!(e)
            }),
        }
    }
}

impl<CommandExecutor> LexLookup<CommandExecutor>
where
    CommandExecutor: Executor,
{
    pub fn new(executor: CommandExecutor) -> Self {
        Self { executor }
    }
}

#[cfg(test)]
mod tests {
    use domain::{arpabet::Arpabet, processor::transcriber::Transcriber};

    use super::LexLookup;
    use crate::command::MockExecutor;

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
