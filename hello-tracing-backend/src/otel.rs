use axum::{
    body::Body,
    http::{HeaderMap, HeaderName},
};
use opentelemetry::{global, propagation::Extractor, trace::TraceContextExt};
use tonic::codegen::http::Request;
use tracing::{error, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

pub fn set_span_parent(request: Request<Body>) -> Request<Body> {
    let span = Span::current();

    let parent_context = global::get_text_map_propagator(|propagator| {
        propagator.extract(&HeaderExtractor(request.headers()))
    });
    span.set_parent(parent_context);

    request
}

pub fn record_trace_id(request: Request<Body>) -> Request<Body> {
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
