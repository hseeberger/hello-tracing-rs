#[tokio::main]
async fn main() {
    // Error logging already happens in `hello_tracing_backend::main`.
    let _ = hello_tracing_backend::main().await;
}
