use std::process::exit;

use adapter::processor::{
    conversion_table::{ConversionTable, KatakanaConverter},
    lex_lookup::{LexLookup, LexLookupExecutor},
};
use server::router;
use service::{arpabet::ArpabetService, katakana::KatakanaService};
use time::macros::format_description;
use tracing_subscriber::{fmt::time::UtcTime, EnvFilter};

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

    let arpabet_service = ArpabetService::new(LexLookup::new(LexLookupExecutor));
    let katakana_service = KatakanaService::new(ConversionTable::new(KatakanaConverter));

    if let Err(err) = router::start(arpabet_service, katakana_service).await {
        tracing::error!("failed to serve:\n{err:?}");
        exit(1);
    };
}
