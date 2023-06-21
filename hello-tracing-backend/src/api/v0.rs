use self::proto::{
    hello_server::{Hello, HelloServer},
    HelloRequest, HelloResponse,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use tonic::{Request, Response, Status};
use tracing::{debug, instrument};

mod proto {
    tonic::include_proto!("hello_tracing_v0");
}

const MSGS: [&str; 2] = [
    "Hello, I'm a tracing demo!",
    "Hello, I'm built with Rust, Axum and tonic!",
];

pub fn hello() -> HelloServer<HelloService> {
    HelloServer::new(HelloService(Arc::new(RwLock::new(0))))
}

pub struct HelloService(Arc<RwLock<usize>>);

#[tonic::async_trait]
impl Hello for HelloService {
    #[instrument(skip(self))]
    async fn hello(
        &self,
        _request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        let mut n = self.0.write().await;
        *n = (*n + 1) % MSGS.len();

        let msg = MSGS[*n];
        debug!(msg, "answering with msg");

        Ok(Response::new(HelloResponse { msg: msg.into() }))
    }
}
