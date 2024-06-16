mod v0;

use anyhow::{Context, Result};
use axum::http::Request;
use hello_tracing_common::otel::http::{accept_trace, record_trace_id};
use serde::Deserialize;
use std::net::IpAddr;
use tokio::signal::unix::{signal, SignalKind};
use tonic::{body::BoxBody, transport::Server};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::{field, info_span, Span};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    addr: IpAddr,
    port: u16,
}

pub async fn serve(config: Config) -> Result<()> {
    let Config { addr, port } = config;

    let app = Server::builder()
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_grpc().make_span_with(make_span))
                .map_request(accept_trace::<BoxBody>)
                .map_request(record_trace_id),
        )
        .add_service(v0::hello());

    app.serve_with_shutdown((addr, port).into(), shutdown_signal())
        .await
        .context("run server")
}

fn make_span<B>(request: &Request<B>) -> Span {
    let headers = request.headers();
    info_span!("incoming request", ?headers, trace_id = field::Empty)
}

async fn shutdown_signal() {
    signal(SignalKind::terminate())
        .expect("install SIGTERM handler")
        .recv()
        .await;
}
