# Local Development Setup

This document provides instructions for setting up and running the Lyn AI Assistant locally for development.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Node.js](https://nodejs.org/) (v18 or later)
- [Docker](https://docs.docker.com/get-docker/) and [Docker Compose](https://docs.docker.com/compose/install/) (optional, for containerized setup)

## Option 1: Running Services Individually

### 1. Start the Backend Service

```bash
# From the project root
cargo run --bin llm-proxy
```

### 2. Start the Embeddings Service

```bash
# From the project root
cargo run --bin embeddings
```

### 3. Start the Frontend

```bash
# From the project root
cd ui
npm install  # Only needed the first time
npm run dev
```

## Option 2: Using the Development Script

We provide a convenient script to start all services at once:

```bash
# From the project root
./dev.sh
```

This script will:
- Start all backend services
- Start the frontend development server
- Log all output to the `logs` directory
- Automatically clean up all processes when you press Ctrl+C

## Option 3: Using Docker Compose

For a fully containerized development environment:

```bash
# From the project root
docker-compose up
```

This will:
- Build and start all services in containers
- Set up the necessary networking between services
- Mount source code as volumes for hot reloading

To rebuild containers after making changes to Dockerfiles:

```bash
docker-compose up --build
```

## Accessing the Application

- Frontend: http://localhost:5173
- Backend API: http://localhost:8080
- WebSocket: ws://localhost:8080/ws/chat

## Environment Variables

The frontend uses environment variables to connect to the backend services. These are set in the `ui/.env` file:

```
VITE_API_BASE_URL=http://localhost:8080
VITE_WS_BASE_URL=ws://localhost:8080
```

For Docker Compose, these are automatically set to use the container names instead of localhost.

## Troubleshooting

### Port Conflicts

If you see errors about ports being in use, you may have another application using those ports. You can change the ports in the `docker-compose.yml` file or stop the conflicting applications.

### Backend Services Not Starting

Check the logs in the `logs` directory for error messages. Common issues include:
- Missing dependencies
- Configuration errors
- Port conflicts

### Frontend Not Connecting to Backend

Ensure that the environment variables in `ui/.env` are set correctly and that the backend services are running.
