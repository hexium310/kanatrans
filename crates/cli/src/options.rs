use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Options {
    pub(crate) word: String,
    #[arg(short, long)]
    pub(crate) arpabet: bool,
    #[arg(short, long, default_value_t = true)]
    pub(crate) katakana: bool,
}
