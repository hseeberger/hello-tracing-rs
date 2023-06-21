mod api;

use anyhow::{Context, Result};
use configured::Configured;
use serde::Deserialize;
use tracing::{error, info};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() {
    if let Err(error) = init_tracing() {
        eprintln!("hello-tracing-backend exited with ERROR: {error}");
    }

    if let Err(ref error) = run().await {
        error!(
            error = format!("{error:#}"),
            backtrace = %error.backtrace(),
            "hello-tracing-backend exited with ERROR"
        );
    };
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Config {
    api: api::Config,
}

async fn run() -> Result<()> {
    let config = Config::load().context("load configuration")?;

    info!(?config, "starting");

    api::serve(config.api).await
}

fn init_tracing() -> Result<()> {
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(fmt::layer().json())
        .try_init()
        .context("initialize tracing subscriber")
}
