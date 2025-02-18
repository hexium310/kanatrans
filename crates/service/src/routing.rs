use std::sync::Arc;

use axum::Router;

use crate::{
    arpabet::{self, ArpabetServiceInterface},
    katakana::{self, KatakanaServiceInterface},
};

pub fn router<ArpabetService, KatakanaService>(
    arpabet_service: ArpabetService,
    katakana_service: KatakanaService,
) -> Router
where
    ArpabetService: ArpabetServiceInterface,
    KatakanaService: KatakanaServiceInterface,
{
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

#[cfg(test)]
mod tests;
