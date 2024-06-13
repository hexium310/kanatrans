use std::process::exit;

use adapter::processor::{
    conversion_table::{ConversionTable, KatakanaConverter},
    lex_lookup::{LexLookup, LexLookupExecutor},
};
use infrastructure::{arpabet::ArpabetService, katakana::KatakanaService};
use time::macros::format_description;
use tracing_subscriber::{fmt::time::UtcTime, EnvFilter};

mod adapter;
mod domain;
mod infrastructure;

#[tokio::main]
async fn main() {
    let local_timer = UtcTime::new(format_description!(
        "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]+[offset_hour]:[offset_minute]"
    ));
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_timer(local_timer)
        .with_file(true)
        .with_line_number(true)
        .init();

    let arpabet_service = ArpabetService::new(LexLookup::new(LexLookupExecutor::new()));
    let katakana_service = KatakanaService::new(ConversionTable::new(KatakanaConverter::new()));

    match infrastructure::router::start(arpabet_service, katakana_service).await {
        Ok(_) => todo!(),
        Err(err) => {
            tracing::error!("failed to serve:\n{err:?}");
            exit(1);
        },
    }
}
