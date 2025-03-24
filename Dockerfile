# --- Stage 1: Build ---
FROM rust:latest AS builder

WORKDIR /usr/src/notecli
COPY . .

RUN cargo build --release


# --- Stage 2: Runtime ---
FROM debian:bookworm-slim

# Install CA certificates if the app ever needs HTTPS or networking
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy the compiled binary from builder stage
COPY --from=builder /usr/src/notecli/target/release/notecli .

# Set default run command
ENTRYPOINT ["./notecli"]
    