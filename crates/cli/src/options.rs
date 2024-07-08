use clap::{Args, Parser};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Options {
    pub(crate) word: String,
    #[command(flatten)]
    pub(crate) output_kind: OutputKindArgs,
}

#[derive(Args)]
pub struct OutputKindArgs {
    #[arg(short, long)]
    pub(crate) arpabet: bool,
    #[arg(short, long, default_value_t = true)]
    pub(crate) katakana: bool,
}

pub enum OutputKind {
    Arpabet,
    Katakana,
}

impl OutputKindArgs {
    pub fn kind(&self) -> OutputKind {
        match (self.arpabet, self.katakana) {
            (true, _) => OutputKind::Arpabet,
            (_, true) => OutputKind::Katakana,
            _ => unreachable!(),
        }
    }
}
