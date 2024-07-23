use anyhow::{Context, Result};
use error_ext::StdErrorExt;
use opentelemetry::{global, trace::TracerProvider, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{propagation::TraceContextPropagator, runtime, trace, Resource};
use serde::Deserialize;
use tracing::{error, Subscriber};
use tracing_subscriber::{
    fmt, layer::SubscriberExt, registry::LookupSpan, util::SubscriberInitExt, EnvFilter, Layer,
};

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    service_name: String,
    otlp_exporter_endpoint: String,
}

/// Initialize tracing: apply an `EnvFilter` using the `RUST_LOG` environment variable to define the
/// log levels, add a formatter layer logging trace events as JSON and on OpenTelemetry layer
/// exporting trace data.
pub fn init_tracing(config: Config) -> Result<()> {
    global::set_text_map_propagator(TraceContextPropagator::new());

    global::set_error_handler(
        |error| error!(target: "otel", error = error.as_chain(), "otel error"),
    )
    .context("set error handler")?;

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(fmt::layer().json().flatten_event(true))
        .with(otlp_layer(config)?)
        .try_init()
        .context("initialize tracing subscriber")
}

/// Create an OTLP layer exporting tracing data.
fn otlp_layer<S>(config: Config) -> Result<impl Layer<S>>
where
    S: Subscriber + for<'span> LookupSpan<'span>,
{
    let exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint(config.otlp_exporter_endpoint);

    let service_name = Resource::new(vec![KeyValue::new(
        "service.name",
        config.service_name.clone(),
    )]);
    let trace_config = trace::Config::default().with_resource(service_name);

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(exporter)
        .with_trace_config(trace_config)
        .install_batch(runtime::Tokio)
        .context("install tracer")?
        .tracer(config.service_name);

    Ok(tracing_opentelemetry::layer().with_tracer(tracer))
}
