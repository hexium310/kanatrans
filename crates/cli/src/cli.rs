use std::{
    io::{stdout, BufWriter, Write},
    ops::Deref,
};

use anyhow::Result;
use clap::Parser;
use service::{arpabet::ArpabetServiceInterface, katakana::KatakanaServiceInterface};

use crate::{options::Options, runner::Runner};

pub struct Cli {
    options: Options,
}

impl Deref for Cli {
    type Target = Options;

    fn deref(&self) -> &Self::Target {
        &self.options
    }
}

impl Default for Cli {
    fn default() -> Self {
        Self {
            options: Options::parse(),
        }
    }
}

impl<ArpabetService, KatakanaService> Runner<ArpabetService, KatakanaService> for Cli
where
    ArpabetService: ArpabetServiceInterface,
    KatakanaService: KatakanaServiceInterface,
{
    async fn run(&self, arpabet_service: ArpabetService, katakana_service: KatakanaService) -> Result<()> {
        let options = &self.options;
        let word = &options.word;

        let arpabet = arpabet_service.get(word.to_owned()).await?;
        let pronunciation = arpabet.pronunciation.iter().map(AsRef::as_ref).collect::<Vec<_>>();

        let stdout = stdout();
        let mut buffer = BufWriter::new(&stdout);

        if options.arpabet {
            writeln!(buffer, "{}", pronunciation.join(" "))?;
            buffer.flush()?;

            return Ok(());
        }

        if options.katakana {
            let katakana = katakana_service.get(&pronunciation).await?;

            writeln!(buffer, "{}", katakana.pronunciation)?;
            buffer.flush()?;
        }

        Ok(())
    }
}
