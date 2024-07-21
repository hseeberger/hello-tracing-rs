mod api;
mod backend;

use crate::backend::Backend;
use anyhow::{Context, Result};
use hello_tracing_common::{config::ConfigExt, log_error, tracing::init_tracing};
use serde::Deserialize;
use std::panic;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    // Load configuration first, because needed for tracing initialization.
    let config = match Config::load().context("load configuration") {
        Ok(config) => config,
        Err(error) => {
            log_error(&error);
            return;
        }
    };

    // If tracing initialization fails, nevertheless emit a structured log event.
    let result = init_tracing(config.tracing.clone());
    if let Err(ref error) = result {
        log_error(error);
        return;
    };

    // Replace the default panic hook with one that uses structured logging at ERROR level.
    panic::set_hook(Box::new(|panic| error!(%panic, "process panicked")));

    // Run and log any error.
    if let Err(ref error) = run(config).await {
        error!(
            error = format!("{error:#}"),
            backtrace = %error.backtrace(),
            "process exited with ERROR"
        );
    }
}

#[derive(Debug, Deserialize)]
struct Config {
    api: api::Config,
    backend: backend::Config,
    tracing: hello_tracing_common::tracing::Config,
}

async fn run(config: Config) -> Result<()> {
    info!(?config, "starting");

    let backend = Backend::new(config.backend);
    api::serve(config.api, backend).await
}
