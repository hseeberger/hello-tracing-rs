ARG RUST_VERSION=1.74.0

FROM rust:${RUST_VERSION}-bullseye AS builder
RUN apt-get update && \
  DEBIAN_FRONTEND=noninteractive apt-get install --no-install-recommends --assume-yes protobuf-compiler
WORKDIR /root/hello-tracing-rs
COPY . .
RUN cargo build --release --package hello-tracing-backend

FROM debian:bullseye-slim
COPY --from=builder /root/hello-tracing-rs/target/release/hello-tracing-backend /usr/local/bin/hello-tracing-backend
COPY --from=builder /root/hello-tracing-rs/hello-tracing-backend/config /opt/hello-tracing-backend/config
WORKDIR /opt/hello-tracing-backend
ENTRYPOINT ["hello-tracing-backend"]
EXPOSE 8080/tcp
