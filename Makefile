SHELL=/bin/bash

rust_version = $(shell grep 'rust-version' Cargo.toml | grep -Eo '\d+\.\d+\.\d+')

check:
	cargo check --tests
	cargo clippy

test:
	cargo test
