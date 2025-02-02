use std::{net::Ipv6Addr, time::Duration};

use anyhow::{Context, Error, Result};
use axum::{body::Body, extract::Request, http::Response, Router};
use tokio::{
    net::TcpListener,
    signal::unix::{signal, SignalKind},
};
use tower::service_fn;
use tower_http::trace::TraceLayer;
use tracing::Span;

pub async fn start() -> Result<()> {
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

    let listener = TcpListener::bind((Ipv6Addr::UNSPECIFIED, port)).await?;

    let app = Router::new()
        .fallback_service(service_fn(|req: Request| async move {
            let router = service::routing::Router::<Body>::default();
            router.route(req).await
        }))
        .layer(trace_layer);

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
