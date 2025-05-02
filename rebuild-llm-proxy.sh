#!/bin/bash

echo "Stopping all containers..."
docker-compose down

echo "Rebuilding the llm-proxy service..."
docker-compose build --no-cache llm-proxy

echo "Starting all services..."
docker-compose up -d

echo "Done! The llm-proxy service has been rebuilt with the latest source code."
