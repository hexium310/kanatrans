use std::process::exit;

use cli::runner::Runner;
use service::{arpabet::ArpabetServiceInterface, katakana::KatakanaServiceInterface};

use crate::command::Command;

pub(crate) async fn run<ArpabetService, KatakanaService>(
    command: Command,
    arpabet_service: ArpabetService,
    katakana_service: KatakanaService,
) where
    ArpabetService: ArpabetServiceInterface,
    KatakanaService: KatakanaServiceInterface,
{
    if let Err(err) = command.cli.run(arpabet_service, katakana_service).await {
        eprintln!("{err}");
        exit(1);
    };
}
