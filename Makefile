build:
	cargo build --workspace

run:
	cargo build --manifest-path=fake_plugin/Cargo.toml
	cargo build --manifest-path=weather_fake_plugin/Cargo.toml
	cargo build --manifest-path=experimental_plugin/Cargo.toml
	cargo run

test: 
	cargo build --manifest-path=fake_plugin/Cargo.toml
	cargo build --manifest-path=weather_fake_plugin/Cargo.toml
	cargo build --manifest-path=experimental_plugin/Cargo.toml
	cargo test

check:
	cargo check --workspace

benchmark: 
	cargo build --release  --manifest-path=fake_plugin/Cargo.toml
	cargo build --release --manifest-path=weather_fake_plugin/Cargo.toml
	cargo build --release --manifest-path=experimental_plugin/Cargo.toml
	cargo bench --manifest-path=bench/Cargo.toml
