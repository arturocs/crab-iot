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

benchmark: 
	cargo build --release  --manifest-path=fake_plugin/Cargo.toml
	cargo bench --manifest-path=bench/Cargo.toml