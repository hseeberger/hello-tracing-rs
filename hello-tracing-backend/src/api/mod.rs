mod v0;

use anyhow::{Context, Result};
use serde::Deserialize;
use std::net::{IpAddr, SocketAddr};
use tokio::signal::unix::{signal, SignalKind};
use tonic::transport::Server;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

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
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_grpc()))
        .add_service(v0::hello());

    app.serve_with_shutdown(socket_addr, shutdown_signal())
        .await
        .context("serving the api")
}

async fn shutdown_signal() {
    signal(SignalKind::terminate())
        .expect("install SIGTERM handler")
        .recv()
        .await;
}
