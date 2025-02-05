use std::sync::Arc;

use adapter::processor::{conversion_table::ConversionTable, lex_lookup::LexLookup};
use axum::Router;

use crate::{
    arpabet::{self, ArpabetService},
    katakana::{self, KatakanaService},
};

pub async fn route() -> Router {
    let arpabet_service = ArpabetService::<LexLookup>::default();
    let katakana_service = KatakanaService::<ConversionTable>::default();

    let arpabet = Router::new()
        .route("/{word}", axum::routing::get(arpabet::handle))
        .with_state(Arc::new(arpabet_service));

    let katakana = Router::new()
        .route("/", axum::routing::get(katakana::handle))
        .with_state(Arc::new(katakana_service));

    Router::new()
        .nest("/arpabet", arpabet)
        .nest("/katakana", katakana)
}
