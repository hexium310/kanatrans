use adapter::processor::{
    conversion_table::{ConversionTable, KatakanaConverter},
    lex_lookup::{LexLookup, LexLookupExecutor},
};
use clap::Parser;
use command::Command;
use service::{arpabet::ArpabetService, katakana::KatakanaService};

mod command;

#[tokio::main]
async fn main() {
    let arpabet_service = ArpabetService::new(LexLookup::new(LexLookupExecutor));
    let katakana_service = KatakanaService::new(ConversionTable::new(KatakanaConverter));

    let command = Command::parse();

    match command.kind() {
        command::CommandKind::Serve => {
            #[cfg(feature = "server")]
            command::serve::run(arpabet_service, katakana_service).await;
        },
        command::CommandKind::Cli => {
            #[cfg(feature = "cli")]
            command::cli::run(command, arpabet_service, katakana_service).await;
        },
    }
}
