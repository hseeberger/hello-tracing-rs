mod v0;

use crate::otel::{accept_trace, record_trace_id};
use anyhow::{Context, Result};
use axum::{body::Body, http::Request};
use serde::Deserialize;
use std::net::{IpAddr, SocketAddr};
use tokio::signal::unix::{signal, SignalKind};
use tonic::transport::Server;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::{field, info_span, Span};

#[derive(Debug, Clone, Deserialize)]
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

pub async fn serve(config: Config) -> Result<()> {
    let socket_addr = config.socket_addr();

    let app = Server::builder()
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_grpc().make_span_with(make_span))
                .map_request(accept_trace)
                .map_request(record_trace_id),
        )
        .add_service(v0::hello());

    app.serve_with_shutdown(socket_addr, shutdown_signal())
        .await
        .context("serving the api")
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
