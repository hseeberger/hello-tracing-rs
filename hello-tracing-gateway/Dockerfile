ARG RUST_VERSION=1.70.0

FROM rust:${RUST_VERSION}-bullseye AS builder
RUN apt-get update && \
  DEBIAN_FRONTEND=noninteractive apt-get install --no-install-recommends --assume-yes protobuf-compiler
WORKDIR /root/hello-tracing-rs
COPY . .
RUN cargo build --release --package hello-tracing-gateway

FROM debian:bullseye-slim
COPY --from=builder /root/hello-tracing-rs/target/release/hello-tracing-gateway /usr/local/bin/hello-tracing-gateway
COPY --from=builder /root/hello-tracing-rs/hello-tracing-gateway/config /opt/hello-tracing-gateway/config
WORKDIR /opt/hello-tracing-gateway
ENTRYPOINT ["hello-tracing-gateway"]
EXPOSE 80/tcp
