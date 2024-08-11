set shell := ["bash", "-uc"]
rust_version := `grep channel rust-toolchain.toml | sed -r 's/channel = "(.*)"/\1/'`

check:
	cargo check -p hello-tracing-common
	cargo check -p hello-tracing-backend
	cargo check -p hello-tracing-gateway

fmt toolchain="+nightly":
	cargo {{toolchain}} fmt

fmt-check toolchain="+nightly":
	cargo {{toolchain}} fmt --check

lint:
	cargo clippy -p hello-tracing-common  --tests --no-deps -- -D warnings
	cargo clippy -p hello-tracing-backend --tests --no-deps -- -D warnings
	cargo clippy -p hello-tracing-gateway --tests --no-deps -- -D warnings

test:
	cargo test -p hello-tracing-common
	cargo test -p hello-tracing-backend
	cargo test -p hello-tracing-gateway

fix:
	cargo fix --tests --allow-dirty --allow-staged

all: check fmt lint test

run-gateway port="8080" backend_port="8081":
	RUST_LOG=hello_tracing_gateway=debug,info \
		CONFIG_FILE=hello-tracing-backend/config.yaml \
		APP__API__PORT={{port}} \
		APP__BACKEND__ENDPOINT=http://localhost:{{backend_port}} \
		cargo run -p hello-tracing-gateway

run-backend port="8081":
	RUST_LOG=hello_tracing_backend=debug,info \
		CONFIG_FILE=hello-tracing-backend/config.yaml \
		APP__API__PORT={{port}} \
		cargo run -p hello-tracing-backend

docker tag="latest":
	docker build \
		--build-arg "RUST_VERSION={{rust_version}}" \
		-t hseeberger/hello-tracing-backend:{{tag}} \
		-f hello-tracing-backend/Dockerfile \
		.
	docker build \
		--build-arg "RUST_VERSION={{rust_version}}" \
		-t hseeberger/hello-tracing-gateway:{{tag}} \
		-f hello-tracing-gateway/Dockerfile \
		.

release level execute="":
	cargo release \
		--exclude hello-tracing-common \
		--sign-commit \
		--sign-tag \
		--no-verify \
		{{level}} {{execute}}
