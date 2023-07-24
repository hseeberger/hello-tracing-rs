mod v0;

use crate::{
    backend::Backend,
    otel::{accept_trace, record_trace_id},
};
use anyhow::{Context, Result};
use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::IntoResponse,
    routing::get,
    Router, Server,
};
use serde::Deserialize;
use std::net::{IpAddr, SocketAddr};
use tokio::signal::unix::{signal, SignalKind};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::{field, info_span, Span};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    addr: IpAddr,
    port: u16,
}

impl Config {
    fn socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.addr, self.port)
    }
}

pub async fn serve(config: Config, backend: Backend) -> Result<()> {
    let app = Router::new()
        .route("/", get(ready))
        .nest("/v0", v0::app(backend))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http().make_span_with(make_span))
                .map_request(accept_trace)
                .map_request(record_trace_id),
        );

    Server::bind(&config.socket_addr())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("serving api")
}

async fn ready() -> impl IntoResponse {
    StatusCode::OK
}

fn make_span(request: &Request<Body>) -> Span {
    let headers = request.headers();
    info_span!("incoming request", ?headers, trace_id = field::Empty)
}

async fn shutdown_signal() {
    signal(SignalKind::terminate())
        .expect("install SIGTERM handler")
        .recv()
        .await;
}
