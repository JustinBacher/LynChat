#!/bin/bash

# Kill any existing processes
pkill -f "llm-proxy"

# Start Docker Compose without the llm-proxy service
echo "Starting Docker Compose..."
docker-compose up --build --remove-orphans --scale llm-proxy=0 -d

# Wait for Docker Compose to start
sleep 5

# Start the llm-proxy service manually
echo "Starting llm-proxy service manually..."
cd services/llm-proxy && cargo run &

# Wait for the llm-proxy service to start
sleep 5

echo "All services started successfully!"
