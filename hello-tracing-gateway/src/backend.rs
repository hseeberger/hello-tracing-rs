mod proto {
    tonic::include_proto!("hello_tracing_v0");
}

use crate::backend::proto::{hello_client::HelloClient, HelloRequest};
use anyhow::{Context, Result};
use serde::Deserialize;
use tracing::debug;

#[derive(Debug, Clone)]
pub struct Backend {
    config: Config,
}

impl Backend {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn hello(&self) -> Result<String> {
        let mut client = HelloClient::connect(self.config.endpoint.to_owned())
            .await
            .with_context(|| format!("connect to endpoint {}", self.config.endpoint))?;

        let msg = client
            .hello(HelloRequest {})
            .await
            .with_context(|| format!("call rpc Hello on endpoint {}", self.config.endpoint))?
            .into_inner()
            .msg;

        debug!(
            msg,
            self.config.endpoint, "received response from rpc Hello"
        );

        Ok(msg)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    endpoint: String,
}
