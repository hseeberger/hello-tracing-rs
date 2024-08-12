# ---------- BUILD STAGE ---------- #

ARG RUST_VERSION
FROM rust:$RUST_VERSION-bookworm AS build_stage

ARG PROFILE=release

WORKDIR /build

# Install protobuf-compiler.
RUN apt-get update && apt-get install -y protobuf-compiler

COPY ./Cargo.toml ./Cargo.lock ./
COPY ./rust-toolchain.toml ./
COPY ./hello-tracing-backend/Cargo.toml ./hello-tracing-backend/
COPY ./hello-tracing-common/Cargo.toml ./hello-tracing-common/
COPY ./hello-tracing-gateway/Cargo.toml ./hello-tracing-gateway/

RUN mkdir ./hello-tracing-backend/src && \
    echo "fn main() {}" > ./hello-tracing-backend/src/main.rs && \
    mkdir ./hello-tracing-common/src && \
    echo "fn main() {}" > ./hello-tracing-common/src/main.rs && \
    mkdir ./hello-tracing-gateway/src && \
    echo "fn main() {}" > ./hello-tracing-gateway/src/main.rs && \
    find . -path '*/src/*' | xargs touch -t 197001010001 -m

RUN cargo build -p hello-tracing-backend --locked --profile $PROFILE && \
    find ./target | xargs touch -t 197001010002 -m

COPY ./ ./

RUN cargo build -p hello-tracing-backend --locked --profile $PROFILE && \
    mv ./target/$([ "$PROFILE" = "release" ] && echo "release" || echo "debug")/hello-tracing-backend /

# ---------- RUNTIME STAGE ---------- #

FROM debian:bookworm-slim

RUN adduser --disabled-password --gecos "" --home "/nonexistent" --shell "/sbin/nologin" --no-create-home --uid "10001" appuser
RUN mkdir /var/run/hello-tracing-backend && chown appuser:appuser /var/run/hello-tracing-backend

COPY --from=build_stage --chown=appuser:appuser /build/hello-tracing-backend/bin/entrypoint.sh /usr/local/bin/
COPY --from=build_stage --chown=appuser:appuser /hello-tracing-backend /usr/local/bin/
COPY --from=build_stage --chown=appuser:appuser /build/hello-tracing-backend/config.yaml /opt/hello-tracing-backend/

USER appuser

WORKDIR /opt/hello-tracing-backend

ENTRYPOINT ["entrypoint.sh"]
