#![feature(result_option_inspect)]

mod api;

use anyhow::{Context, Result};
use configured::Configured;
use hello_tracing_common::{
    log_error,
    tracing::{init_tracing, TracingConfig},
};
use serde::Deserialize;
use std::panic;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration first, because needed for tracing initialization.
    let config = Config::load()
        .context("load configuration")
        .inspect_err(log_error)?;

    // If tracing initialization fails, nevertheless emit a structured log event.
    init_tracing(config.tracing.clone()).inspect_err(log_error)?;

    // Replace the default panic hook with one that uses structured logging at ERROR level.
    panic::set_hook(Box::new(|panic| error!(%panic, "process panicked")));

    // Run and log any error.
    run(config).await.inspect_err(|error| {
        error!(
            error = format!("{error:#}"),
            backtrace = %error.backtrace(),
            "process exited with ERROR"
        );
    })
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Config {
    api: api::Config,
    tracing: TracingConfig,
}

async fn run(config: Config) -> Result<()> {
    info!(?config, "starting");

    api::serve(config.api).await
}
