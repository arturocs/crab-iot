build:
	cargo build --workspace

run:
	cargo build --release --manifest-path=fake_plugin/Cargo.toml
	cargo run

test: 
	cargo build --manifest-path=fake_plugin/Cargo.toml
	cargo test

check:
	cargo check --workspace

benchmark: 
	cargo build --release  --manifest-path=fake_plugin/Cargo.toml
	cargo bench --manifest-path=bench/Cargo.toml