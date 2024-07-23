mod proto {
    tonic::include_proto!("hello_tracing_backend_v0");
}

use self::proto::{
    hello_server::{Hello, HelloServer},
    HelloRequest, HelloResponse,
};
use std::sync::atomic::{AtomicBool, Ordering};
use tonic::{Request, Response, Status};
use tracing::{debug, instrument};

const MESSAGES: [&str; 2] = [
    "Hello, I'm a tracing demo!",
    "Hello, I'm built with Rust, Axum and tonic!",
];

pub fn hello() -> HelloServer<HelloService> {
    HelloServer::new(HelloService(AtomicBool::default()))
}

pub struct HelloService(AtomicBool);

#[tonic::async_trait]
impl Hello for HelloService {
    #[instrument(name = "hello-handler", skip(self, _request))]
    async fn hello(
        &self,
        _request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        // Toggle message index.
        let previous = self.0.load(Ordering::Acquire);
        let current = !previous;
        self.0.store(current, Ordering::Release);

        // Get message for this response.
        let message = if current { MESSAGES[0] } else { MESSAGES[1] };
        let message = message.to_string();
        debug!(message, "answering");

        Ok(Response::new(HelloResponse { message }))
    }
}
