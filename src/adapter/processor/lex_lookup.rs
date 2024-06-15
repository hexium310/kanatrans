use std::process::Command;

use anyhow::{anyhow, Result};

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

        let convert_error = anyhow!("cannot convert to ARPAbet");

        let arpabet = output
            .lines()
            .next()
            .ok_or(&convert_error)
            .unwrap()
            .trim_matches(|char| ['(', ')'].contains(&char))
            .split_whitespace()
            .map(ToString::to_string)
            .collect::<Vec<_>>();

        if arpabet.first().unwrap() == "[null]" {
            return Err(convert_error);
        }

        Ok(Arpabet::new(&arpabet))
    }
}

impl Executor for LexLookupExecutor {
    fn execute(&self, word: &str) -> Result<String> {
        let output = Command::new("lex_lookup").arg(word).output()?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
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
            .returning(|_| Ok("(th r eh1 sh ow1 l d)\n(th r eh1 sh hh ow1 l d)".to_string()));

        let lex_lookup = LexLookup::new(mock_executor);
        let arpabet = lex_lookup.transcribe("threshold").unwrap();

        assert_eq!(
            arpabet,
            Arpabet(["th", "r", "eh1", "sh", "ow1", "l", "d"].map(Into::into).to_vec())
        )
    }
}
