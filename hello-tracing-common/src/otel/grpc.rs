use opentelemetry::propagation::Injector;
use tonic::{
    metadata::{MetadataKey, MetadataMap, MetadataValue},
    Request, Status,
};
use tracing::{error, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

/// Trace context propagation: send the trace context by injecting it into the metadata of the given
/// request.
pub fn send_trace<T>(mut request: Request<T>) -> Result<Request<T>, Status> {
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
