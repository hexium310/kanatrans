use std::io::{stdout, BufWriter, Write};

use anyhow::Result;
use clap::Parser;
use service::{arpabet::ArpabetServiceInterface, katakana::KatakanaServiceInterface};

use crate::{
    options::{Options, OutputKind},
    runner::Runner,
};

pub struct Cli {
    options: Options,
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

        let stdout = stdout();
        let mut buffer = BufWriter::new(&stdout);

        for word in &options.words {
            let arpabet = arpabet_service.get(word.to_owned()).await?;
            let pronunciation = arpabet.pronunciation.iter().map(AsRef::as_ref).collect::<Vec<_>>();

            match options.output_kind.kind() {
                OutputKind::Arpabet => {
                    writeln!(buffer, "{}", pronunciation.join(" "))?;
                    buffer.flush()?;
                },
                OutputKind::Katakana => {
                    let katakana = katakana_service.get(&pronunciation).await?;

                    writeln!(buffer, "{}", katakana.pronunciation)?;
                    buffer.flush()?;
                },
                OutputKind::All => {
                    let katakana = katakana_service.get(&pronunciation).await?;

                    writeln!(buffer, "{}\t{}", pronunciation.join(" "), katakana.pronunciation)?;
                    buffer.flush()?;
                },
            };
        }

        Ok(())
    }
}
