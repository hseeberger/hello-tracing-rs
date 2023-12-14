set shell := ["bash", "-uc"]

check:
	@echo "RUSTUP_TOOLCHAIN is ${RUSTUP_TOOLCHAIN:-not set}"
	cargo check --tests

fmt:
	@echo "RUSTUP_TOOLCHAIN is ${RUSTUP_TOOLCHAIN:-not set}"
	cargo fmt

fmt-check:
	@echo "RUSTUP_TOOLCHAIN is ${RUSTUP_TOOLCHAIN:-not set}"
	cargo fmt --check

lint:
	@echo "RUSTUP_TOOLCHAIN is ${RUSTUP_TOOLCHAIN:-not set}"
	cargo clippy --no-deps -- -D warnings

test:
	@echo "RUSTUP_TOOLCHAIN is ${RUSTUP_TOOLCHAIN:-not set}"
	cargo test

fix:
	@echo "RUSTUP_TOOLCHAIN is ${RUSTUP_TOOLCHAIN:-not set}"
	cargo fix --allow-dirty --allow-staged

all: check fmt lint test

run-gateway port="8080" backend_port="8081":
	RUST_LOG=hello_tracing_gateway=debug,info \
		CONFIG_DIR=hello-tracing-gateway/config \
		APP__API__PORT={{port}} \
		APP__BACKEND__ENDPOINT=http://localhost:{{backend_port}} \
		cargo run -p hello-tracing-gateway

run-backend port="8081":
	RUST_LOG=hello_tracing_backend=debug,info \
		CONFIG_DIR=hello-tracing-backend/config \
		APP__API__PORT={{port}} \
		cargo run -p hello-tracing-backend

docker tag="latest":
	docker build \
		-t hseeberger/hello-tracing-backend:{{tag}} \
		-f hello-tracing-backend/Dockerfile \
		.
	docker build \
		-t hseeberger/hello-tracing-gateway:{{tag}} \
		-f hello-tracing-gateway/Dockerfile \
		.
