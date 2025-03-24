# Use official Rust image
FROM rust:latest

# Create app directory inside container
WORKDIR /usr/src/notecli

# Copy the source code
COPY . .

# Build the app
RUN cargo build --release

# Default run command (can be overridden)
ENTRYPOINT ["./target/release/notecli"]
