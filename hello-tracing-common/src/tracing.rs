use anyhow::{Context, Result};
use opentelemetry::{
    global, runtime,
    sdk::{propagation::TraceContextPropagator, trace, Resource},
    trace::TraceContextExt,
    KeyValue,
};
use opentelemetry_otlp::WithExportConfig;
use serde::Deserialize;
use std::{collections::HashMap, iter};
use tracing::{error, Subscriber};
use tracing_opentelemetry::OtelData;
use tracing_subscriber::{
    fmt, layer::SubscriberExt, registry::LookupSpan, util::SubscriberInitExt, EnvFilter, Layer,
};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TracingConfig {
    service_name: String,
    otlp_exporter_endpoint: String,
}

pub fn init_tracing(config: TracingConfig) -> Result<()> {
    global::set_text_map_propagator(TraceContextPropagator::new());

    global::set_error_handler(|error| error!(error = format!("{error:#}"), "otel error"))
        .context("set error handler")?;

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(
            fmt::layer()
                .json()
                .with_span_list(false)
                .with_extra_fields(Box::new(|extensions| {
                    extensions
                        .get::<OtelData>()
                        .map(|otel_data| {
                            let trace_id = otel_data.parent_cx.span().span_context().trace_id();
                            HashMap::from_iter(iter::once((
                                "trace_id".to_string(),
                                trace_id.to_string(),
                            )))
                        })
                        .unwrap_or_default()
                })),
        )
        .with(otel_layer(config)?)
        .try_init()
        .context("initialize tracing subscriber")
}

/// Create an OpenTelemetry tracing layer
fn otel_layer<S>(config: TracingConfig) -> Result<impl Layer<S>>
where
    S: Subscriber + for<'span> LookupSpan<'span>,
{
    let exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint(config.otlp_exporter_endpoint);

    let trace_config = trace::config().with_resource(Resource::new(vec![KeyValue::new(
        "service.name",
        config.service_name,
    )]));

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(exporter)
        .with_trace_config(trace_config)
        .install_batch(runtime::Tokio)
        .context("install tracer")?;

    Ok(tracing_opentelemetry::layer().with_tracer(tracer))
}
