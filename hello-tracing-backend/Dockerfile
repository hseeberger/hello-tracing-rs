ARG RUST_VERSION=1.79.0

FROM rust:${RUST_VERSION}-slim-bookworm AS builder
WORKDIR /app
COPY . .
RUN \
  apt-get update && \
  apt-get install -y jq protobuf-compiler
RUN \
  --mount=type=cache,target=/app/target/ \
  --mount=type=cache,target=/usr/local/cargo/registry/ \
  cargo build --package hello-tracing-backend --release && \
  cp ./target/release/hello-tracing-backend /

FROM debian:bookworm-slim AS final
RUN adduser \
  --disabled-password \
  --gecos "" \
  --home "/nonexistent" \
  --shell "/sbin/nologin" \
  --no-create-home \
  --uid "10001" \
  appuser
COPY --from=builder /hello-tracing-backend /usr/local/bin
RUN chown appuser /usr/local/bin/hello-tracing-backend
COPY --from=builder /app/hello-tracing-backend/config /opt/hello-tracing-backend/config
RUN chown -R appuser /opt/hello-tracing-backend
USER appuser
ENV RUST_LOG="hello_tracing_backend=debug,info"
WORKDIR /opt/hello-tracing-backend
ENTRYPOINT ["hello-tracing-backend"]
EXPOSE 8080/tcp
