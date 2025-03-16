mod v0;

use crate::backend::Backend;
use anyhow::{Context, Result};
use api_version::{ApiVersionLayer, ApiVersions};
use axum::{
    body::Body,
    http::{Request, StatusCode, Uri},
    response::IntoResponse,
    routing::get,
    Router, ServiceExt,
};
use hello_tracing_common::otel::{accept_trace, record_trace_id};
use serde::Deserialize;
use std::{convert::Infallible, net::IpAddr};
use tokio::{
    net::TcpListener,
    signal::unix::{signal, SignalKind},
};
use tower::{Layer, ServiceBuilder};
use tower_http::trace::TraceLayer;
use tracing::{field, info_span, trace_span, Span};

const API_VERSIONS: ApiVersions<1> = ApiVersions::new([0]);

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub address: IpAddr,
    pub port: u16,
}

pub async fn serve(config: Config, backend: Backend) -> Result<()> {
    let Config { address, port } = config;

    let app = Router::new()
        .route("/", get(ready))
        .nest("/v0", v0::app(backend))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http().make_span_with(make_span))
                .map_request(accept_trace)
                .map_request(record_trace_id),
        );
    let app = ApiVersionLayer::new(API_VERSIONS, ApiVersionFilter).layer(app);

    let listener = TcpListener::bind((address, port))
        .await
        .context("bind TcpListener")?;
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("run server")
}

#[derive(Clone)]
struct ApiVersionFilter;

impl api_version::ApiVersionFilter for ApiVersionFilter {
    type Error = Infallible;

    async fn filter(&self, uri: &Uri) -> Result<bool, Self::Error> {
        Ok(uri.path() != "/")
    }
}

async fn ready() -> impl IntoResponse {
    StatusCode::OK
}

fn make_span(request: &Request<Body>) -> Span {
    let headers = request.headers();

    let path = request.uri().path();

    // Disable (well, silence) spans/traces for root spans (readiness checks).
    if path.is_empty() || path == "/" {
        trace_span!("incoming request", path, ?headers, trace_id = field::Empty)
    } else {
        info_span!("incoming request", path, ?headers, trace_id = field::Empty)
    }
}

async fn shutdown_signal() {
    signal(SignalKind::terminate())
        .expect("install SIGTERM handler")
        .recv()
        .await;
}
