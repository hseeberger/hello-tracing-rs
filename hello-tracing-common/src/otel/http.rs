use axum::{body::Body, http};
use opentelemetry::{global, trace::TraceContextExt};
use opentelemetry_http::HeaderExtractor;
use tracing::Span;
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
