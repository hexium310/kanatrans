use adapter::processor::{
    conversion_table::{ConversionTable, KatakanaConverter},
    lex_lookup::{LexLookup, LexLookupExecutor},
};
use cli::{runner::Runner, Cli};
use service::{arpabet::ArpabetService, katakana::KatakanaService};

#[tokio::main]
async fn main() {
    let cli = Cli::default();
    let arpabet_service = ArpabetService::new(LexLookup::new(LexLookupExecutor));
    let katakana_service = KatakanaService::new(ConversionTable::new(KatakanaConverter));

    if let Err(err) = cli.run(arpabet_service, katakana_service).await {
        eprintln!("{err}");
    };
}
