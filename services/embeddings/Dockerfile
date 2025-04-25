FROM rust:1.76-slim as builder

WORKDIR /app

# Install dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy the entire workspace
COPY . .

# Build the application
RUN cargo build --bin embeddings

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y libssl-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /app/target/debug/embeddings /app/embeddings

# Expose the port
EXPOSE 8080

# Run the binary
CMD ["/app/embeddings"]
