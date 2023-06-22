use axum::{
    body::Body,
    http::{self, HeaderMap, HeaderName},
};
use opentelemetry::{global, propagation::Extractor, trace::TraceContextExt};
use tracing::{error, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

/// Associate the current span with the OTel trace of the given request.
pub fn associate_span_with_trace(request: http::Request<Body>) -> http::Request<Body> {
    let span = Span::current();

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

struct HeaderExtractor<'a>(&'a HeaderMap);

impl<'a> Extractor for HeaderExtractor<'a> {
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|value| match value.to_str() {
            Ok(value) => Some(value),

            Err(error) => {
                error!(
                    error = format!("{error:#}"),
                    "convert header value to valid ASCII",
                );
                None
            }
        })
    }

    fn keys(&self) -> Vec<&str> {
        self.0.keys().map(HeaderName::as_str).collect()
    }
}
