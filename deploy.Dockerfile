FROM rust:slim

WORKDIR app

COPY . .

RUN apt-get update && apt-get install make; make build; cargo build --release

ENTRYPOINT ["./target/release/crab-iot"]
