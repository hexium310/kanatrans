use std::process::exit;

use server::router;
use service::{arpabet::ArpabetServiceInterface, katakana::KatakanaServiceInterface};
use time::macros::format_description;
use tracing_subscriber::{EnvFilter, fmt::time::UtcTime};

pub(crate) async fn run<ArpabetService, KatakanaService>(
    arpabet_service: ArpabetService,
    katakana_service: KatakanaService,
) where
    ArpabetService: ArpabetServiceInterface,
    KatakanaService: KatakanaServiceInterface,
{
    let local_timer = UtcTime::new(format_description!(
        "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]+[offset_hour]:[offset_minute]"
    ));
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_timer(local_timer)
        .init();

    if let Err(err) = router::start(arpabet_service, katakana_service).await {
        tracing::error!("failed to serve:\n{err:?}");
        exit(1);
    };
}
