use adapter::processor::{
    conversion_table::{ConversionTable, KatakanaConverter},
    lex_lookup::{LexLookup, LexLookupExecutor},
};
use infrastructure::{arpabet::ArpabetService, katakana::KatakanaService};

mod adapter;
mod domain;
mod infrastructure;

#[tokio::main]
async fn main() {
    let arpabet_service = ArpabetService::new(LexLookup::new(LexLookupExecutor::new()));
    let katakana_service = KatakanaService::new(ConversionTable::new(KatakanaConverter::new()));

    match infrastructure::router::start(arpabet_service, katakana_service).await {
        Ok(_) => todo!(),
        Err(err) => {
            tracing::error!("failed to serve:\n{err:?}")
        },
    }
}
