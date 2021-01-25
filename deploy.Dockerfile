FROM rust:1.49

WORKDIR app

COPY . .

RUN make build; cargo build --release

ENTRYPOINT ["./target/release/crab-iot"]