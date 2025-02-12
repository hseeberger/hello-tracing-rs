use opentelemetry::{trace::TracerProvider as _, KeyValue};
use opentelemetry_otlp::{SpanExporter, WithExportConfig};
use opentelemetry_sdk::{
    propagation::TraceContextPropagator, runtime, trace::TracerProvider, Resource,
};
use serde::Deserialize;
use thiserror::Error;
use tracing::error;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Telemetry (logging, tracing, metrics) configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(rename = "tracing")]
    pub tracing_config: TracingConfig,
}

/// Tracing (as opposed to logging or metrics) configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct TracingConfig {
    pub enabled: bool,
    pub service_name: String,
    pub otlp_exporter_endpoint: String,
}

/// Error possibly returned by [init].
#[derive(Debug, Error)]
pub enum Error {
    #[error("cannot initialize tracing subscriber")]
    TryInitTracing(#[from] tracing_subscriber::util::TryInitError),

    #[error("cannot install OTLP tracer")]
    InstallOtlpTracer(#[from] opentelemetry::trace::TraceError),
}

/// Initialize telemetry: apply an `EnvFilter` using the `RUST_LOG` environment variable to define
/// the log levels, add a formatter layer logging as JSON and an OpenTelemetry layer exporting
/// tracing data if tracing is enabled.
pub fn init(config: Config) -> Result<(), Error> {
    let Config { tracing_config } = config;

    let tracing = tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer().json().flatten_event(true));

    // The below little code duplication is needed because `tracing` and
    // `tracing.with(otlp_layer(config)?)` have different types.
    if tracing_config.enabled {
        opentelemetry::global::set_text_map_propagator(TraceContextPropagator::new());
        tracing.with(otlp_layer(tracing_config)?).try_init()?
    } else {
        tracing.try_init()?
    }

    Ok(())
}

/// Create an OTLP layer exporting tracing data.
fn otlp_layer<S>(config: TracingConfig) -> Result<impl tracing_subscriber::Layer<S>, Error>
where
    S: tracing::Subscriber + for<'span> tracing_subscriber::registry::LookupSpan<'span>,
{
    let exporter = SpanExporter::builder()
        .with_tonic()
        .with_endpoint(config.otlp_exporter_endpoint)
        .build()?;

    let service_name = Resource::new(vec![KeyValue::new(
        "service.name",
        config.service_name.clone(),
    )]);

    let provider = TracerProvider::builder()
        .with_resource(service_name)
        .with_batch_exporter(exporter, runtime::Tokio)
        .build();

    let tracer = provider.tracer("config.service_name");

    Ok(tracing_opentelemetry::layer().with_tracer(tracer))
}

#[cfg(test)]
mod tests {
    use crate::telemetry::{self, Config, TracingConfig};

    #[tokio::test]
    async fn test_init() {
        let tracing_config = TracingConfig {
            enabled: true,
            service_name: "test".to_string(),
            otlp_exporter_endpoint: "http://localhost:4317".to_string(),
        };
        let config = Config { tracing_config };
        let result = telemetry::init(config);
        assert!(result.is_ok());

        let tracing_config = TracingConfig {
            enabled: false,
            service_name: "test".to_string(),
            otlp_exporter_endpoint: "http://localhost:4317".to_string(),
        };
        let config = Config { tracing_config };
        let result = telemetry::init(config);
        assert!(result.is_err());
    }
}
