# /home/inno/elights_jobes-research/backend/core-api/Dockerfile

# --- Stage 1: Builder ---
# Use official Rust image with desired toolchain version
FROM rust:1.75 as builder

# Set working directory
WORKDIR /usr/src/app

# Install build dependencies if needed (e.g., for libraries needing C compilation like openssl, postgres)
# RUN apt-get update && apt-get install -y build-essential pkg-config libssl-dev libpq-dev clang

# Create dummy project to cache dependencies
RUN USER=root cargo new --bin .
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
# Copy workspace Cargo.toml if dependencies are defined there
COPY ../../Cargo.toml ./root-Cargo.toml
# Build dependencies only (will cache layers)
RUN cargo build --release
RUN rm src/*.rs target/release/deps/core_api* # Remove dummy artifacts

# Copy all source code from the workspace required for core-api build
# Adjust paths based on your actual workspace structure
COPY ./src ./src
COPY ../domain ./domain # Copy domain crate source
COPY ../../bank-integrations ./bank-integrations # Copy bank integrations source
COPY ../../cryptography-exchange ./cryptography-exchange # Copy crypto exchange source
COPY ../../database ./database # Copy database schema/migrations if needed at build time

# Build the actual application binary in release mode
# Ensure workspace dependencies are correctly referenced in Cargo.toml
RUN cargo build --release --bin core-api

# --- Stage 2: Runtime ---
# Use a minimal base image
FROM debian:bullseye-slim

# Set arguments for user/group (optional, good practice)
ARG APP_USER=appuser
ARG APP_GROUP=appgroup
ARG UID=1001
ARG GID=1001

# Create non-root user and group
RUN groupadd -g ${GID} ${APP_GROUP} && \
    useradd -u ${UID} -g ${APP_GROUP} -m -s /sbin/nologin ${APP_USER}

# Install runtime dependencies (e.g., ca-certificates for TLS, libpq for postgres if dynamically linked)
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    # libpq5 # Needed if diesel dynamically links libpq
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/core-api /usr/local/bin/core-api

# Copy migration files if migrations are run from within the container on start (optional)
# COPY --from=builder /usr/src/app/database/migrations ./database/migrations
# COPY --from=builder /usr/src/app/diesel.toml ./diesel.toml # Needed for diesel migration run

# Set ownership to non-root user (optional)
# RUN chown -R ${APP_USER}:${APP_GROUP} /app /usr/local/bin/core-api ./database ./diesel.toml
# USER ${APP_USER}

# Expose the port the application listens on (from .env)
# Defaulting to 8080, but relies on environment variable being set correctly at runtime
EXPOSE 8080
# EXPOSE 8443 # If TLS is also exposed

# Set the entrypoint command
# Consider using an entrypoint script for setup tasks (like running migrations) before starting the app
# ENTRYPOINT ["./entrypoint.sh"]
CMD ["core-api"]