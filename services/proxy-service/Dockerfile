# --- Chef stage (base for all build stages) ---
FROM rust:1.86-slim-bookworm AS chef
WORKDIR /app

# Install cargo-chef
RUN cargo install cargo-chef

# Install build dependencies
RUN apt-get update && \
    apt-get install -y \
        libssl-dev pkg-config libclang-dev clang cmake git \
        libomp-dev libgomp1 llvm-14 libomp5-14 libomp-14-dev && \
    rm -rf /var/lib/apt/lists/*

RUN rustup component add rustfmt

ENV CXX=clang++
ENV CC=clang

# --- Planner stage ---
FROM chef AS planner
WORKDIR /app

# Copy the source code for recipe preparation
COPY services/llm-proxy /app
COPY common /app/common

# Create the recipe
RUN cargo chef prepare --recipe-path recipe.json

# --- Builder stage ---
FROM chef AS builder
WORKDIR /app

ARG BUILD_ENV=development
ENV BUILD_ENV=${BUILD_ENV}

# Copy the recipe from the planner
COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the key step that will be cached
RUN cargo chef cook --recipe-path recipe.json

# Copy the actual source code
COPY services/llm-proxy /app
COPY common /app/common

# Build the application
RUN cargo build

# --- Runtime stage ---
FROM debian:bookworm-slim
WORKDIR /app

ARG BUILD_ENV=development
ENV BUILD_ENV=${BUILD_ENV}

RUN apt-get update && \
    apt-get install -y \
        libssl-dev ca-certificates libomp-dev \
        libgomp1 llvm-14 libomp5-14 && \
    rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /app/target/debug/llm-proxy /app/llm-proxy

EXPOSE 8083
ENV SERVER_HOST=0.0.0.0
ENV RUST_LOG=debug
CMD ["/app/llm-proxy"]
