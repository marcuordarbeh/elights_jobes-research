# Dockerfile for building the Rust backend.
FROM rust:latest as builder

WORKDIR /usr/src/payment-system

# Copy Cargo.toml and Cargo.lock
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Use a minimal base image for the runtime
FROM debian:buster-slim

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/payment-system

# Copy the compiled binary from builder
COPY --from=builder /usr/src/payment-system/target/release/payment-system .

EXPOSE 8080

CMD ["./payment-system"]
