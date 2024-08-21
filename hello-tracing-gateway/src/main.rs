#[tokio::main]
async fn main() {
    // Error logging already happens in `hello_tracing_gateway::main`.
    let _ = hello_tracing_gateway::main().await;
}
