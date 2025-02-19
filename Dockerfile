FROM rust:latest as builder

WORKDIR /usr/src/group-generator

COPY Cargo.toml Cargo.lock ./

COPY . .

RUN cargo build --release

FROM ubuntu:22.04

WORKDIR /usr/src/group-generator

# Sets a HOME environment variable to a directory inside the the container
ENV HOME=/usr/src/group-generator/data

# Creates the directory for saving the app  state after the program have been run and set permissions
RUN mkdir -p $HOME && chown -R root:root $HOME

COPY --from=builder /usr/src/group-generator/target/release/group-generator .

# Runs the executable on container startup
CMD ["./group-generator"]
