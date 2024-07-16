use std::io::{stdout, BufWriter, Write};

use anyhow::Result;
use clap::Args;
use service::{arpabet::ArpabetServiceInterface, katakana::KatakanaServiceInterface};

use crate::runner::Runner;

#[derive(Args)]
pub struct Options {
    /// Words composed of English alphabet
    pub words: Vec<String>,
    #[command(flatten)]
    pub(crate) output_kind: OutputKindArgs,
}

#[derive(Args)]
pub struct OutputKindArgs {
    /// Print APRAbet
    #[arg(short, long)]
    pub(crate) arpabet: bool,
    /// Print Katakana (default: true)
    #[arg(short, long)]
    pub(crate) katakana: bool,
}

pub enum OutputKind {
    Arpabet,
    Katakana,
    All,
}

impl OutputKindArgs {
    pub fn kind(&self) -> OutputKind {
        match (self.arpabet, self.katakana) {
            (true, false) => OutputKind::Arpabet,
            (false, true) => OutputKind::Katakana,
            (true, true) => OutputKind::All,
            (false, false) => OutputKind::Katakana,
        }
    }
}

impl<ArpabetService, KatakanaService> Runner<ArpabetService, KatakanaService> for Options
where
    ArpabetService: ArpabetServiceInterface,
    KatakanaService: KatakanaServiceInterface,
{
    async fn run(&self, arpabet_service: ArpabetService, katakana_service: KatakanaService) -> Result<()> {
        let stdout = stdout();
        let mut buffer = BufWriter::new(&stdout);

        for word in &self.words {
            let arpabet = arpabet_service.get(word.to_owned()).await?;
            let pronunciation = arpabet.pronunciation.iter().map(AsRef::as_ref).collect::<Vec<_>>();

            match self.output_kind.kind() {
                OutputKind::Arpabet => {
                    writeln!(buffer, "{}", pronunciation.join(" "))?;
                },
                OutputKind::Katakana => {
                    let katakana = katakana_service.get(&pronunciation).await?;

                    writeln!(buffer, "{}", katakana.pronunciation)?;
                },
                OutputKind::All => {
                    let katakana = katakana_service.get(&pronunciation).await?;

                    writeln!(buffer, "{} ({})", katakana.pronunciation, pronunciation.join(" "))?;
                },
            };
        }

        buffer.flush()?;

        Ok(())
    }
}
