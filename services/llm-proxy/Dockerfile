FROM debian:stable

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y libssl-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# The binary will be mounted from the shared volume

# Expose the port
EXPOSE 8083

# Set environment variables
ENV SERVER_HOST=0.0.0.0
ENV RUST_LOG=debug

# Run the binary from the mounted volume with explicit host binding
CMD ["sh", "-c", "RUST_BACKTRACE=1 /app/target/debug/llm-proxy"]
