use std::process::exit;

use server::router;
use time::macros::format_description;
use tracing_subscriber::{fmt::time::UtcTime, EnvFilter};

pub(crate) async fn run() {
    let local_timer = UtcTime::new(format_description!(
        "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]+[offset_hour]:[offset_minute]"
    ));
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_timer(local_timer)
        .init();

    if let Err(err) = router::start().await {
        tracing::error!("failed to serve:\n{err:?}");
        exit(1);
    };
}
