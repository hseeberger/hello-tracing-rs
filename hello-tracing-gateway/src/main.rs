mod api;
mod backend;

use crate::backend::Backend;
use anyhow::{Context, Result};
use configured::Configured;
use hello_tracing_common::tracing::{init_tracing, TracingConfig};
use serde::Deserialize;
use tracing::error;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load().context("load configuration")?;
    init_tracing(config.tracing).context("initialize tracing")?;

    let backend = Backend::new(config.backend);
    let result = api::serve(config.api, backend).await;

    if let Err(error) = &result {
        error!(
            error = format!("{error:#}"),
            backtrace = %error.backtrace(),
            "hello-tracing-gateway exited with ERROR"
        );
    };
    result
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Config {
    api: api::Config,
    backend: backend::Config,
    tracing: TracingConfig,
}
