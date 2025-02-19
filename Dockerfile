FROM rust:latest as builder

WORKDIR /usr/src/group-generator

COPY Cargo.toml Cargo.lock ./

COPY . .

RUN cargo build --release

FROM ubuntu:22.04

WORKDIR /usr/src/group-generator

COPY --from=builder /usr/src/group-generator/target/release/group-generator .

# Runs the executable on container startup
CMD ["./group-generator"]
