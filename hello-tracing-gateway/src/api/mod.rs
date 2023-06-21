mod v0;

use crate::backend::Backend;
use anyhow::{Context, Result};
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router, Server};
use serde::Deserialize;
use std::net::{IpAddr, SocketAddr};
use tokio::signal::unix::{signal, SignalKind};
use tower::ServiceBuilder;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

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
                .layer(TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::new())),
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

async fn shutdown_signal() {
    signal(SignalKind::terminate())
        .expect("install SIGTERM handler")
        .recv()
        .await;
}
