use adapter::processor::{conversion_table::ConversionTable, lex_lookup::LexLookup};
use infrastructure::{alpabet::AlpabetService, katakana::KatakanaService};

mod adapter;
mod domain;
mod infrastructure;

#[tokio::main]
async fn main() {
    let alpabet_service = AlpabetService::new(LexLookup::new());
    let katakana_service = KatakanaService::new(ConversionTable::new());

    match infrastructure::router::start(alpabet_service, katakana_service).await {
        Ok(_) => todo!(),
        Err(err) => {
            tracing::error!("failed to serve:\n{err:?}")
        },
    }
}
