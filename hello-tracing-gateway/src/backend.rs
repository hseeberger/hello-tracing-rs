mod proto {
    tonic::include_proto!("hello_tracing_backend_v0");
}

use self::proto::{hello_client::HelloClient, HelloRequest};
use anyhow::{Context, Result};
use hello_tracing_common::otel::grpc::send_trace;
use serde::Deserialize;
use std::str::FromStr;
use tonic::transport::Endpoint;
use tracing::{debug, instrument};

#[derive(Debug, Clone)]
pub struct Backend {
    config: Config,
}

impl Backend {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    #[instrument(name = "hello-backend-client", skip(self))]
    pub async fn hello(&self) -> Result<String> {
        let endpoint = Endpoint::from_str(&self.config.endpoint)
            .with_context(|| format!("create endpoint {}", self.config.endpoint))?;
        let channel = endpoint
            .connect()
            .await
            .with_context(|| format!("connect to endpoint {}", self.config.endpoint))?;
        let mut client = HelloClient::with_interceptor(channel, send_trace);

        let msg = client
            .hello(HelloRequest {})
            .await
            .with_context(|| format!("call rpc Hello on endpoint {}", self.config.endpoint))?
            .into_inner()
            .msg;

        debug!(
            msg,
            endpoint = self.config.endpoint,
            "received response from rpc Hello"
        );

        Ok(msg)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    endpoint: String,
}
