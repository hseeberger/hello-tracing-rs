use axum::{body::Body, http};
use opentelemetry::{global, propagation::Injector, trace::TraceContextExt};
use opentelemetry_http::HeaderExtractor;
use tonic::metadata::{MetadataKey, MetadataMap, MetadataValue};
use tracing::{error, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

/// Associate the current span with the OTel trace of the given request, if any and valid.
pub fn accept_trace(request: http::Request<Body>) -> http::Request<Body> {
    let span = Span::current();

    // Current context, if no or invalid data is received.
    let parent_context = global::get_text_map_propagator(|propagator| {
        propagator.extract(&HeaderExtractor(request.headers()))
    });
    span.set_parent(parent_context);

    request
}

/// Recorcd the OTel trace ID of the given request as "trace_id" field in the current span.
pub fn record_trace_id(request: http::Request<Body>) -> http::Request<Body> {
    let span = Span::current();

    let trace_id = span.context().span().span_context().trace_id();
    span.record("trace_id", trace_id.to_string());

    request
}

/// Propagate the OTel trace by injecting the trace context into the metadata of the given request.
pub fn propagate_trace<T>(
    mut request: tonic::Request<T>,
) -> Result<tonic::Request<T>, tonic::Status> {
    opentelemetry::global::get_text_map_propagator(|propagator| {
        let context = Span::current().context();
        propagator.inject_context(&context, &mut MetadataInjector(request.metadata_mut()))
    });
    Ok(request)
}

struct MetadataInjector<'a>(&'a mut MetadataMap);

impl Injector for MetadataInjector<'_> {
    fn set(&mut self, key: &str, value: String) {
        match MetadataKey::from_bytes(key.as_bytes()) {
            Ok(name) => match MetadataValue::try_from(&value) {
                Ok(value) => {
                    self.0.insert(name, value);
                }

                Err(error) => error!(
                    value,
                    error = format!("{error:#}"),
                    "parse value as metadata value"
                ),
            },

            Err(error) => error!(
                key,
                error = format!("{error:#}"),
                "parse key as metadata value"
            ),
        }
    }
}
