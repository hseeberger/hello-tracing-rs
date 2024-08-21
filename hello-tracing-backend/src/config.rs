use crate::api;
use hello_tracing_common::telemetry;
use serde::Deserialize;

/// The main configuration hosting the application specific [Config] and the [telemetry::Config].
#[derive(Debug, Clone, Deserialize)]
pub struct MainConfig {
    #[serde(flatten)]
    pub config: Config,

    #[serde(rename = "telemetry")]
    pub telemetry_config: telemetry::Config,
}

/// The application sepcific configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(rename = "api")]
    pub api_config: api::Config,
}
