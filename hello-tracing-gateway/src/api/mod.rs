mod v0;

use crate::backend::Backend;
use anyhow::{Context, Result};
use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::IntoResponse,
    routing::get,
    Router, Server,
};
use hello_tracing_common::otel::http::{accept_trace, record_trace_id};
use serde::Deserialize;
use std::{net::IpAddr, time::Duration};
use tokio::{
    signal::unix::{signal, SignalKind},
    time,
};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::{field, info_span, trace_span, Span};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    addr: IpAddr,
    port: u16,
    #[serde(with = "humantime_serde")]
    shutdown_timeout: Option<Duration>,
}

pub async fn serve(config: Config, backend: Backend) -> Result<()> {
    let Config {
        addr,
        port,
        shutdown_timeout,
    } = config;

    let app = Router::new()
        .route("/", get(ready))
        .nest("/v0", v0::app(backend))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http().make_span_with(make_span))
                .map_request(accept_trace)
                .map_request(record_trace_id),
        );

    Server::bind(&(addr, port).into())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal(shutdown_timeout))
        .await
        .context("run server")
}

async fn ready() -> impl IntoResponse {
    StatusCode::OK
}

fn make_span(request: &Request<Body>) -> Span {
    let headers = request.headers();

    let path = request.uri().path();

    // Disable (well, silence) spans/traces for root spans.
    if path.is_empty() || path == "/" {
        trace_span!("incoming request", ?headers, trace_id = field::Empty)
    } else {
        info_span!("incoming request", ?headers, trace_id = field::Empty)
    }
}

async fn shutdown_signal(shutdown_timeout: Option<Duration>) {
    signal(SignalKind::terminate())
        .expect("install SIGTERM handler")
        .recv()
        .await;
    if let Some(shutdown_timeout) = shutdown_timeout {
        time::sleep(shutdown_timeout).await;
    }
}
