.PHONY: build run test clean docs

all: build

build:
	cargo build

release:
	cargo build --release

run:
	cargo run

run-release:
	cargo run --release

test:
	cargo test

docs:
	cargo doc --open

clean:
	cargo clean

fmt:
	cargo fmt

check:
	cargo check

clippy:
	cargo clippy 