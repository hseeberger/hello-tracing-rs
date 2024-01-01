use axum::{
    body::Body,
    http::{HeaderMap, Request},
};
use opentelemetry::{global, propagation::Extractor, trace::TraceContextExt};
use tracing::{warn, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

/// Trace context propagation: associate the current span with the OTel trace of the given request,
/// if any and valid.
pub fn accept_trace(request: Request<Body>) -> Request<Body> {
    // Current context, if no or invalid data is received.
    let parent_context = global::get_text_map_propagator(|propagator| {
        propagator.extract(&HeaderExtractor(request.headers()))
    });
    Span::current().set_parent(parent_context);

    request
}

/// Recorcd the OTel trace ID of the given request as "trace_id" field in the current span.
pub fn record_trace_id(request: Request<Body>) -> Request<Body> {
    let span = Span::current();

    let trace_id = span.context().span().span_context().trace_id();
    span.record("trace_id", trace_id.to_string());

    request
}

struct HeaderExtractor<'a>(&'a HeaderMap);

impl<'a> Extractor for HeaderExtractor<'a> {
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|v| {
            let s = v.to_str();
            if let Err(ref error) = s {
                warn!(%error, ?v, "cannot convert header value to ASCII")
            };
            s.ok()
        })
    }

    fn keys(&self) -> Vec<&str> {
        self.0.keys().map(|k| k.as_str()).collect()
    }
}
