SHELL=/bin/bash

rust_version = $(shell grep 'rust-version' Cargo.toml | grep -Eo '\d+\.\d+\.\d+')

check:
	cargo check --tests
	cargo clippy

test:
	cargo test

docker:
	docker build \
		--build-arg RUST_VERSION=1.70.0 \
		-t hseeberger/hello-tracing-backend \
		-f hello-tracing-backend/Dockerfile \
		.
	docker build \
		--build-arg RUST_VERSION=1.70.0 \
		-t hseeberger/hello-tracing-gateway \
		-f hello-tracing-gateway/Dockerfile \
		.
