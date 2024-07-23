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
    endpoint: String,
}

impl Backend {
    pub fn new(config: Config) -> Self {
        let Config { endpoint } = config;
        Self { endpoint }
    }

    #[instrument(name = "hello-backend-client", skip(self))]
    pub async fn hello(&self) -> Result<String> {
        let endpoint = Endpoint::from_str(&self.endpoint)
            .with_context(|| format!("create endpoint {}", self.endpoint))?;
        let channel = endpoint
            .connect()
            .await
            .with_context(|| format!("connect to endpoint {}", self.endpoint))?;
        let mut client = HelloClient::with_interceptor(channel, send_trace);

        let message = client
            .hello(HelloRequest {})
            .await
            .with_context(|| format!("call Hello on endpoint {}", self.endpoint))?
            .into_inner()
            .message;

        debug!(
            message,
            endpoint = self.endpoint,
            "received response from Hello"
        );

        Ok(message)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    endpoint: String,
}
