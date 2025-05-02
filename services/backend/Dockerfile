# --- Builder stage ---
FROM debian:bookworm-slim AS builder
WORKDIR /app

ARG BUILD_ENV=development
ENV BUILD_ENV=${BUILD_ENV}

# Create a placeholder script
RUN mkdir -p /app/bin
RUN echo '#!/bin/sh\necho "Backend service placeholder"' > /app/bin/backend
RUN chmod +x /app/bin/backend

# --- Runtime stage ---
FROM debian:bookworm-slim
WORKDIR /app

ARG BUILD_ENV=development
ENV BUILD_ENV=${BUILD_ENV}

RUN apt-get update && \
    apt-get install -y libssl-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/bin/backend /app/backend

EXPOSE 8080
CMD ["/app/backend"]
