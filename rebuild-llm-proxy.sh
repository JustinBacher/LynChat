#!/bin/bash

echo "Stopping all containers..."
docker-compose down

echo "Rebuilding the builder image to ensure it has the latest source code..."
docker-compose build --no-cache builder

echo "Starting the builder to compile the latest code..."
docker-compose run --rm builder cargo clean --package llm-proxy
docker-compose run --rm builder cargo build --bin llm-proxy

echo "Starting all services..."
docker-compose up -d

echo "Done! The llm-proxy service has been rebuilt with the latest source code."
