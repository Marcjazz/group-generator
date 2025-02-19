FROM rust:alpine AS build-env

WORKDIR /usr/src/group-generator

COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Install necessary build dependencies
RUN apk add --no-cache musl-dev upx

RUN cargo build --release

# Compress the binary using upx
RUN upx --best target/release/group-generator

# Create the directory for saving the apps state
RUN mkdir -p /usr/src/group-generator/data

# Use a minimal base image for the final image
FROM scratch

WORKDIR /usr/src/group-generator

# Set the HOME environment variable to a directory which saves the apps state in the container
ENV HOME=/usr/src/group-generator/data

# Copy the directory for saving state
COPY --from=build-env /usr/src/group-generator/data /usr/src/group-generator/data

# Copy the statically linked and compressed binary from the builder stage
COPY --from=build-env /usr/src/group-generator/target/release/group-generator .

CMD ["./group-generator"]
