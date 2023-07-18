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
