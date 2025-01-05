FROM rust:1.83-slim-bookworm as builder

WORKDIR /usr/src/app

# Install OpenSSL development packages
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy the entire project
COPY crates/haos_orderbook crates/haos_orderbook

COPY Cargo.toml Cargo.lock ./

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /usr/local/bin

# Install OpenSSL runtime libraries
RUN apt-get update && \
    apt-get install -y openssl ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /usr/src/app/target/release/haos_orderbook .

# Run the binary
CMD ["haos_orderbook"]
