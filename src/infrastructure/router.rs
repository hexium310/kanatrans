use std::{sync::Arc, time::Duration};

use anyhow::{Context, Result};
use axum::{
    http::{Request, Response},
    routing::get,
    Router,
};
use tokio::{
    net::TcpListener,
    signal::unix::{signal, SignalKind},
};
use tower_http::trace::{DefaultOnFailure, TraceLayer};
use tracing::{Level, Span};

use super::service::{arpabet::ArpabetServiceInterface, katakana::KatakanaServiceInterface};
use crate::infrastructure::service::{arpabet, katakana};

pub async fn start<ArpabetService, KatakanaService>(
    arpabet_service: ArpabetService,
    katakana_service: KatakanaService,
) -> Result<()>
where
    ArpabetService: ArpabetServiceInterface,
    KatakanaService: KatakanaServiceInterface,
{
    let trace_layer = TraceLayer::new_for_http()
        .on_request(|request: &Request<_>, _span: &Span| {
            tracing::info!("request: {} {}", request.method(), request.uri())
        })
        .on_response(|response: &Response<_>, latency: Duration, _span: &Span| {
            tracing::info!("response: {} in {latency:?}", response.status())
        })
        .on_failure(DefaultOnFailure::new().level(Level::ERROR));

    let arpabet = Router::new()
        .route("/:word", get(arpabet::get))
        .with_state(Arc::new(arpabet_service));

    let katakana = Router::new()
        .route("/:word", get(katakana::get))
        .with_state(Arc::new(katakana_service));

    let app = Router::new()
        .nest("/arpabet", arpabet)
        .nest("/katakana", katakana)
        .layer(trace_layer);

    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    tracing::debug!(
        "listening on {}",
        listener.local_addr().context("failed to return local address")?
    );

    tokio::spawn(async move {
        axum::serve(listener, app.into_make_service()).await.unwrap();
    });

    let mut interrupt = signal(SignalKind::interrupt())?;
    let mut terminate = signal(SignalKind::terminate())?;

    tokio::select! {
        _ = interrupt.recv() => {},
        _ = terminate.recv() => {},
    };

    Ok(())
}
