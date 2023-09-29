set shell := ["bash", "-uc"]

rust_version := `grep 'rust-version' Cargo.toml | grep -Eo '\d+\.\d+\.\d+'`

check:
	cargo check --tests

fmt:
	cargo +nightly fmt

lint:
	cargo clippy --no-deps -- -D warnings

test:
	cargo test

all: fmt check lint test

run-gateway:
	RUST_LOG=hello_tracing_gateway=debug,info \
		CONFIG_DIR=hello-tracing-gateway/config \
		APP__API__PORT=8080 \
		APP__BACKEND__ENDPOINT=http://localhost:8090 \
		cargo run -p hello-tracing-gateway \
		> $HOME/tmp/hello-tracing-gateway.log

run-backend:
	RUST_LOG=hello_tracing_backend=debug,info \
		CONFIG_DIR=hello-tracing-backend/config \
		APP__API__PORT=8090 \
		cargo run -p hello-tracing-backend \
		> $HOME/tmp/hello-tracing-backend.log

docker:
	docker build \
		--build-arg RUST_VERSION={{rust_version}} \
		-t hseeberger/hello-tracing-backend \
		-f hello-tracing-backend/Dockerfile \
		.
	docker build \
		--build-arg RUST_VERSION={{rust_version}} \
		-t hseeberger/hello-tracing-gateway \
		-f hello-tracing-gateway/Dockerfile \
		.
