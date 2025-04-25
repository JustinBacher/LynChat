#!/bin/bash
# Development script to run all services locally

# Exit on error
set -e

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}Starting Lyn AI Assistant development environment...${NC}"

# Create a directory for logs if it doesn't exist
mkdir -p logs

# Set up environment variables for services
# Embeddings service environment variables
export QDRANT_URL="http://localhost:6333"
export SERVER_HOST="127.0.0.1"
export SERVER_PORT="8082"

# Database service environment variables
export DATABASE_URL="postgres://postgres:postgres@localhost:5432/lyn"
export DB_PORT="8081"

# Backend service environment variables
export API_PORT="8080"

# Set Rust logging level
export RUST_LOG="info"

# Function to start a service in the background
start_service() {
    local service_name=$1
    local command=$2
    local log_file="logs/${service_name}.log"

    echo -e "${YELLOW}Starting ${service_name}...${NC}"
    $command > $log_file 2>&1 &
    local pid=$!
    echo $pid > "logs/${service_name}.pid"
    echo -e "${GREEN}${service_name} started with PID $pid${NC}"

    # Check if process is still running after a short delay
    sleep 1
    if ! ps -p $pid > /dev/null; then
        echo -e "${RED}${service_name} failed to start. Check logs at ${log_file}${NC}"
        cat $log_file
    fi
}

# Function to clean up on exit
cleanup() {
    echo -e "${YELLOW}Shutting down services...${NC}"
    for pid_file in logs/*.pid; do
        if [ -f "$pid_file" ]; then
            pid=$(cat $pid_file)
            service_name=$(basename $pid_file .pid)
            echo -e "${YELLOW}Stopping ${service_name} (PID: $pid)${NC}"
            kill $pid 2>/dev/null || true
            rm $pid_file
        fi
    done

    # Ask if Docker containers should be stopped
    read -p "Do you want to stop the Docker containers (PostgreSQL and Qdrant)? (y/n): " stop_containers
    if [[ "$stop_containers" == "y" || "$stop_containers" == "Y" ]]; then
        echo -e "${YELLOW}Stopping Docker containers...${NC}"
        docker stop postgres qdrant 2>/dev/null || true
        echo -e "${GREEN}Docker containers stopped${NC}"
    else
        echo -e "${YELLOW}Docker containers left running${NC}"
    fi

    echo -e "${GREEN}All services stopped${NC}"
}

# Register cleanup function to run on exit
trap cleanup EXIT

# Function to start or restart a Docker container
start_docker_container() {
    local container_name=$1
    local run_command=$2

    # Check if container exists
    if docker ps -a --format '{{.Names}}' | grep -q "^${container_name}$"; then
        echo -e "${YELLOW}Container ${container_name} already exists. Checking if it's running...${NC}"

        # Check if container is running
        if docker ps --format '{{.Names}}' | grep -q "^${container_name}$"; then
            echo -e "${GREEN}Container ${container_name} is already running.${NC}"
        else
            echo -e "${YELLOW}Container ${container_name} exists but is not running. Starting it...${NC}"
            docker start ${container_name} || {
                echo -e "${RED}Failed to start container ${container_name}.${NC}"
                echo -e "${YELLOW}You may need to remove it with: docker rm ${container_name}${NC}"
                return 1
            }
        fi
    else
        echo -e "${YELLOW}Container ${container_name} does not exist. Creating and starting it...${NC}"
        eval ${run_command} || {
            echo -e "${RED}Failed to create and start container ${container_name}.${NC}"
            return 1
        }
    fi

    return 0
}

# Check if PostgreSQL is running (required for database service)
if ! pg_isready -h localhost -p 5432 > /dev/null 2>&1; then
    echo -e "${YELLOW}PostgreSQL is not running. Checking Docker...${NC}"

    start_docker_container "postgres" "docker run -d --name postgres -e POSTGRES_PASSWORD=postgres -e POSTGRES_DB=lyn -p 5432:5432 postgres:14" || {
        echo -e "${RED}Failed to start PostgreSQL. Make sure Docker is running and try again.${NC}"
        echo -e "${YELLOW}You can manually start PostgreSQL with: docker run -d --name postgres -e POSTGRES_PASSWORD=postgres -e POSTGRES_DB=lyn -p 5432:5432 postgres:14${NC}"
        echo -e "${YELLOW}Continuing without PostgreSQL. Database service may fail.${NC}"
    }

    echo -e "${YELLOW}Waiting for PostgreSQL to initialize...${NC}"
    sleep 5
fi

# Check if Qdrant is running (required for embeddings service)
if ! curl -s http://localhost:6333/health > /dev/null; then
    echo -e "${YELLOW}Qdrant vector database is not running. Checking Docker...${NC}"

    start_docker_container "qdrant" "docker run -d --name qdrant -p 6333:6333 -p 6334:6334 qdrant/qdrant" || {
        echo -e "${RED}Failed to start Qdrant. Make sure Docker is running and try again.${NC}"
        echo -e "${YELLOW}You can manually start Qdrant with: docker run -d --name qdrant -p 6333:6333 -p 6334:6334 qdrant/qdrant${NC}"
        echo -e "${YELLOW}Continuing without Qdrant. Embeddings service may fail.${NC}"
    }

    echo -e "${YELLOW}Waiting for Qdrant to initialize...${NC}"
    sleep 5
fi

# Start backend services
start_service "backend" "cargo run --bin llm-proxy"
echo -e "${YELLOW}Waiting for backend to initialize...${NC}"
sleep 2

# Start database service if it exists
# Note: The database service doesn't have a binary target, so we'll use the main.rs directly
if [ -d "services/database" ] && [ -f "services/database/src/main.rs" ]; then
    # Check if PostgreSQL is actually running before starting the database service
    if pg_isready -h localhost -p 5432 > /dev/null 2>&1; then
        echo -e "${GREEN}PostgreSQL is running. Starting database service...${NC}"
        start_service "database" "cd services/database && cargo run"
        echo -e "${YELLOW}Waiting for database to initialize...${NC}"
        sleep 2
    else
        echo -e "${RED}PostgreSQL is not running. Skipping database service.${NC}"
        echo -e "${YELLOW}You can start PostgreSQL manually and then run the database service.${NC}"
    fi
else
    echo -e "${YELLOW}Database service not found or doesn't have a main.rs. Skipping...${NC}"
fi

# Start embeddings service
start_service "embeddings" "cargo run --bin embeddings"
echo -e "${YELLOW}Waiting for embeddings to initialize...${NC}"
sleep 2

# Start frontend development server
# First check if node_modules exists, if not run npm install
if [ ! -d "ui/node_modules" ]; then
    echo -e "${YELLOW}Node modules not found. Running npm install in ui directory...${NC}"
    (cd ui && npm install)
fi

start_service "frontend" "cd ui && npm run dev"
echo -e "${YELLOW}Waiting for frontend to initialize...${NC}"
sleep 3

# Check if services are still running
echo -e "${YELLOW}Checking if services are running...${NC}"
all_running=true

for pid_file in logs/*.pid; do
    if [ -f "$pid_file" ]; then
        pid=$(cat $pid_file)
        service_name=$(basename $pid_file .pid)
        if ! ps -p $pid > /dev/null; then
            echo -e "${RED}${service_name} is not running. Check logs at logs/${service_name}.log${NC}"
            all_running=false
        else
            echo -e "${GREEN}${service_name} is running with PID $pid${NC}"
        fi
    fi
done

if [ "$all_running" = true ]; then
    echo -e "${GREEN}All services are running!${NC}"
    echo -e "${GREEN}Frontend should be available at: http://localhost:5173${NC}"
    echo -e "${GREEN}Backend API should be available at: http://localhost:8080${NC}"
    echo -e "${GREEN}Embeddings API should be available at: http://localhost:8082${NC}"
else
    echo -e "${RED}Some services failed to start. Check the logs for details.${NC}"
fi

echo -e "${YELLOW}Press Ctrl+C to stop all services${NC}"

# Keep the script running until Ctrl+C
wait
