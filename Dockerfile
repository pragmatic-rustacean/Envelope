# Builder stage.
# We use the latest stable rust release as the base image.
FROM lukemathwalker/cargo-chef:latest-rust-1.91.0 AS chef

# Switch our working directory to app.
# This directory will be created by docker if it doesn't already exist.
WORKDIR /app

# Install the required system dependencies for our linking configuration.
RUN apt update && apt install lld clang -y

FROM chef AS planner

# Copy the manifest files to compute the dependency graph.
COPY Cargo.toml Cargo.lock ./
# Compute a lock-file for this project.
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

# Copy the recipe.json file from the planner stage.
COPY --from=planner /app/recipe.json recipe.json
# Build our project deps not our application
RUN cargo chef cook --release --recipe-path recipe.json
# Copy all files from our working environment to our Docker image.
COPY . .
ENV SQLX_OFFLINE=true
# Let's build our binary
# I'll use the release profile to make it faaast.
RUN cargo build --release --bin newslatter

# Runtime stage
FROM debian:bookworm AS runtime

WORKDIR /path

# Install OpenSSL => It is dynamically linked to some of our deps.
# Install ca-certificates => It is needed to verify TLS certificate when establishing connections.
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# copy the compiled binary from the builder environment to our runtime environment.
COPY --from=builder /app/target/release/newslatter newsletter

# We need configuration file at runtime.
COPY config config
ENV APP_ENVIRONMENT=production

# When 'docker run' is executed launch the binary
ENTRYPOINT ["./newsletter"]