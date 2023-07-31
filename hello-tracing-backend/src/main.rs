mod api;

use anyhow::{Context, Result};
use configured::Configured;
use hello_tracing_common::tracing::{init_tracing, TracingConfig};
use serde::Deserialize;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load().context("load configuration")?;
    init_tracing(config.tracing.clone()).context("initialize tracing")?;

    info!(?config, "starting");

    let result = api::serve(config.api).await;

    if let Err(error) = result {
        error!(
            error = format!("{error:#}"),
            backtrace = %error.backtrace(),
            "hello-tracing-backend exited with ERROR"
        );
    };
    Ok(())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Config {
    api: api::Config,
    tracing: TracingConfig,
}
