ARG RUST_VERSION=1.75.0

FROM rust:${RUST_VERSION}-slim-bookworm AS builder
WORKDIR /app
COPY . .
RUN \
  apt-get update && \
  apt-get install -y jq protobuf-compiler
RUN \
  --mount=type=cache,target=/app/target/ \
  --mount=type=cache,target=/usr/local/cargo/registry/ \
  cargo build --package hello-tracing-gateway --release && \
  cp ./target/release/hello-tracing-gateway /

FROM debian:bookworm-slim AS final
RUN adduser \
  --disabled-password \
  --gecos "" \
  --home "/nonexistent" \
  --shell "/sbin/nologin" \
  --no-create-home \
  --uid "10001" \
  appuser
COPY --from=builder /hello-tracing-gateway /usr/local/bin
RUN chown appuser /usr/local/bin/hello-tracing-gateway
COPY --from=builder /app/hello-tracing-gateway/config /opt/hello-tracing-gateway/config
RUN chown -R appuser /opt/hello-tracing-gateway
USER appuser
ENV RUST_LOG="hello_tracing_gateway=debug,info"
WORKDIR /opt/hello-tracing-gateway
ENTRYPOINT ["hello-tracing-gateway"]
EXPOSE 8080/tcp
