set shell := ["bash", "-uc"]

check:
	@echo "RUSTUP_TOOLCHAIN is ${RUSTUP_TOOLCHAIN:-not set}"
	cargo check --features axum --tests
	cargo check --features poem-openapi --tests

fmt:
	@echo "RUSTUP_TOOLCHAIN is ${RUSTUP_TOOLCHAIN:-not set}"
	cargo fmt

fmt-check:
	@echo "RUSTUP_TOOLCHAIN is ${RUSTUP_TOOLCHAIN:-not set}"
	cargo fmt --check

lint:
	@echo "RUSTUP_TOOLCHAIN is ${RUSTUP_TOOLCHAIN:-not set}"
	cargo clippy --all-features --no-deps -- -D warnings

test:
	@echo "RUSTUP_TOOLCHAIN is ${RUSTUP_TOOLCHAIN:-not set}"
	cargo test --all-features

fix:
	@echo "RUSTUP_TOOLCHAIN is ${RUSTUP_TOOLCHAIN:-not set}"
	cargo fix --allow-dirty --allow-staged

all: check fmt lint test

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
		-t hseeberger/hello-tracing-backend \
		-f hello-tracing-backend/Dockerfile \
		.
	docker build \
		-t hseeberger/hello-tracing-gateway \
		-f hello-tracing-gateway/Dockerfile \
		.
