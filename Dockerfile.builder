FROM rust:latest

WORKDIR /app

# Install dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy only the files needed for building
# First, copy the Cargo files which define dependencies
COPY Cargo.toml Cargo.lock ./

# Copy the source code for each service and common libraries
COPY services/database/Cargo.toml services/database/
COPY services/backend/Cargo.toml services/backend/
COPY services/llm-proxy/Cargo.toml services/llm-proxy/
COPY services/embeddings/Cargo.toml services/embeddings/
COPY common/Cargo.toml common/
COPY cli/Cargo.toml cli/
COPY desktop/Cargo.toml desktop/

# Create dummy source files to satisfy Cargo
RUN mkdir -p services/database/src && \
    mkdir -p services/backend/src && \
    mkdir -p services/llm-proxy/src && \
    mkdir -p services/embeddings/src && \
    mkdir -p common/src && \
    mkdir -p cli/src && \
    mkdir -p desktop/src && \
    echo "fn main() {}" > services/database/src/main.rs && \
    echo "fn main() {}" > services/backend/src/main.rs && \
    echo "fn main() {}" > services/llm-proxy/src/main.rs && \
    echo "fn main() {}" > services/embeddings/src/main.rs && \
    echo "fn main() {}" > cli/src/main.rs && \
    echo "fn main() {}" > desktop/src/main.rs && \
    echo "pub fn dummy() {}" > common/src/lib.rs

# Do a preliminary build to cache dependencies
RUN cargo build --bin database --bin backend --bin llm-proxy --bin embeddings

# Now copy the actual source code
COPY services/database/src services/database/src
COPY services/backend/src services/backend/src
COPY services/llm-proxy/src services/llm-proxy/src
COPY services/embeddings/src services/embeddings/src
COPY common/src common/src
COPY cli/src cli/src
COPY desktop/src desktop/src

# Build all binaries with the actual source code
RUN cargo build --bin database --bin backend --bin llm-proxy --bin embeddings

# The binaries will be in /app/target/debug/ and mounted to the shared volume
