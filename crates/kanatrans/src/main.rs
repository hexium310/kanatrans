use adapter::processor::{conversion_table::ConversionTable, lex_lookup::LexLookup};
use clap::Parser;
use command::Command;
use service::{arpabet::ArpabetService, katakana::KatakanaService};

mod command;

#[tokio::main]
async fn main() {
    let arpabet_service = ArpabetService::<LexLookup>::default();
    let katakana_service = KatakanaService::<ConversionTable>::default();

    let command = Command::parse();

    match command.kind() {
        command::CommandKind::Serve => {
            #[cfg(feature = "server")]
            command::serve::run().await;
        },
        command::CommandKind::Cli => {
            #[cfg(feature = "cli")]
            command::cli::run(command, arpabet_service, katakana_service).await;
        },
    }
}
