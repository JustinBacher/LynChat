FROM debian:stable

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y libssl-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# The binary will be mounted from the shared volume

# Expose the port
EXPOSE 8080

# Run the binary from the mounted volume
CMD ["/app/target/debug/embeddings"]
