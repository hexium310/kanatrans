use clap::{Args, Parser};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Options {
    pub(crate) words: Vec<String>,
    #[command(flatten)]
    pub(crate) output_kind: OutputKindArgs,
}

#[derive(Args)]
pub struct OutputKindArgs {
    #[arg(short, long)]
    pub(crate) arpabet: bool,
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
