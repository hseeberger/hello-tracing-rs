mod api;
mod backend;
mod otel;

use crate::backend::Backend;
use anyhow::{Context, Result};
use configured::Configured;
use opentelemetry::{
    global, runtime,
    sdk::{propagation::TraceContextPropagator, trace, Resource},
    KeyValue,
};
use opentelemetry_otlp::WithExportConfig;
use serde::Deserialize;
use tracing::{error, info, warn, Subscriber};
use tracing_subscriber::{
    fmt, layer::SubscriberExt, registry::LookupSpan, util::SubscriberInitExt, EnvFilter, Layer,
};

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load().context("load configuration")?;
    init_tracing(config.tracing).context("initialize tracing")?;

    let result = run().await;
    if let Err(error) = &result {
        error!(
            error = format!("{error:#}"),
            backtrace = %error.backtrace(),
            "hello-tracing-backend exited with ERROR"
        );
    };
    result
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Config {
    api: api::Config,
    backend: backend::Config,
    tracing: TracingConfig,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct TracingConfig {
    service_name: String,
    otlp_exporter_endpoint: String,
}

async fn run() -> Result<()> {
    let config = Config::load().context("load configuration")?;

    info!(?config, "starting");

    let backend = Backend::new(config.backend);
    api::serve(config.api, backend).await
}

fn init_tracing(config: TracingConfig) -> Result<()> {
    global::set_text_map_propagator(TraceContextPropagator::new());

    global::set_error_handler(|error| error!(error = format!("{error:#}"), "otel error"))
        .context("set error handler")?;

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(fmt::layer().json())
        .with(otel_layer(config)?)
        .try_init()
        .context("initialize tracing subscriber")
}

/// Create an OpenTelemetry tracing layer
fn otel_layer<S>(config: TracingConfig) -> Result<impl Layer<S>>
where
    S: Subscriber + for<'span> LookupSpan<'span>,
{
    let trace_config = trace::config().with_resource(Resource::new(vec![KeyValue::new(
        "service.name",
        config.service_name,
    )]));

    let exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint(config.otlp_exporter_endpoint);

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_trace_config(trace_config)
        .with_exporter(exporter)
        .install_batch(runtime::Tokio)
        .context("install tracer")?;

    Ok(tracing_opentelemetry::layer().with_tracer(tracer))
}
