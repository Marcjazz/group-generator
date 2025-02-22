Smallest rust image FROM rust:alpine AS build-env

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN apk add --no-cache musl-dev upx


RUN cargo build --release

RUN upx --best target/release/group-generator

RUN rm -rf ~/.cargo/registry

FROM scratch

COPY --from=build-env /app/target/release/group-generator .

ENTRYPOINT ["/group-generator"]
