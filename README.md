# hello-tracing-rs

Simple dockerized Rust/Axum/toic based tracing demo.

## Run the backend

From the workspace root directory:

```
RUST_LOG=hello_tracing_backend=debug,info \
  CONFIG_DIR=hello-tracing-backend/config \
  APP__API__PORT=8090 \
  cargo run -p hello-tracing-backend \
  > /Users/heiko/tmp/hello-tracing-rs/hello-tracing-backend.log
```

## Run the gateway

From the workspace root directory:

```
RUST_LOG=hello_tracing_gateway=debug,info \
  CONFIG_DIR=hello-tracing-gateway/config \
  APP__API__PORT=8080 \
  APP__BACKEND__ENDPOINT=http://localhost:8090 \
  cargo run -p hello-tracing-gateway \
  > /Users/heiko/tmp/hello-tracing-rs/hello-tracing-gateway.log
```

## License ##

This code is open source software licensed under the [Apache 2.0 License](http://www.apache.org/licenses/LICENSE-2.0.html).
