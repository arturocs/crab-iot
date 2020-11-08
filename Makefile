build:
	cargo build --workspace

run:
	cargo build --workspace
	cargo run

test: 
	cargo build --workspace
	cargo test

check:
	cargo check --workspace
