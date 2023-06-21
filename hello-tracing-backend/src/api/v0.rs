use self::proto::{
    hello_server::{Hello, HelloServer},
    HelloRequest, HelloResponse,
};
use tonic::{Request, Response, Status};

mod proto {
    tonic::include_proto!("hello_tracing_v0");
}

pub fn hello() -> HelloServer<HelloService> {
    HelloServer::new(HelloService)
}

pub struct HelloService;

#[tonic::async_trait]
impl Hello for HelloService {
    async fn hello(
        &self,
        _request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        Ok(Response::new(HelloResponse {
            msg: "Hello, I'm a Rust/Axum/tonic service!".into(),
        }))
    }
}
