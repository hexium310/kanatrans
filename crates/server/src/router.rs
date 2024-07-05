use std::{net::Ipv6Addr, sync::Arc, time::Duration};

use anyhow::{Context, Error, Result};
use axum::{http::Response, routing::get, Router};
use service::{arpabet::ArpabetServiceInterface, katakana::KatakanaServiceInterface};
use tokio::{
    net::TcpListener,
    signal::unix::{signal, SignalKind},
};
use tower_http::trace::TraceLayer;
use tracing::Span;

use crate::{arpabet, katakana};

pub async fn start<ArpabetService, KatakanaService>(
    arpabet_service: ArpabetService,
    katakana_service: KatakanaService,
) -> Result<()>
where
    ArpabetService: ArpabetServiceInterface,
    KatakanaService: KatakanaServiceInterface,
{
    let trace_layer = TraceLayer::new_for_http()
        .on_request(())
        .on_response(|response: &Response<_>, latency: Duration, _span: &Span| {
            tracing::info!("response {} in {latency:?}", response.status())
        })
        .on_failure(());

    let port = std::env::var("KANATRANS_PORT")
        .map_err(Error::from)
        .and_then(|port| port.parse::<u16>().map_err(Error::from))
        .context("KANATRANS_PORT should be set port")?;

    let arpabet = Router::new()
        .route("/:word", get(arpabet::get))
        .with_state(Arc::new(arpabet_service));

    let katakana = Router::new()
        .route("/", get(katakana::get))
        .with_state(Arc::new(katakana_service));

    let app = Router::new()
        .nest("/arpabet", arpabet)
        .nest("/katakana", katakana)
        .layer(trace_layer);

    let listener = TcpListener::bind((Ipv6Addr::UNSPECIFIED, port)).await?;

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
