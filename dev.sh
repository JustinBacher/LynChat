#!/bin/bash
# Docker-based development script for Lyn AI Assistant

# Exit on error
set -e

export COMPOSE_BAKE=true
export COMPOSE_DOCKER_CLI_BUILD=1
export DOCKER_BUILDKIT=1

# Colors for output (fallback if gum is not available)
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Check if gum is installed and install it if not found
check_and_install_gum() {
  if ! command -v gum &> /dev/null; then
    # We can't use print_styled here since it depends on gum
    echo -e "${YELLOW}Gum CLI not found. Would you like to install it for a more polished UI? (y/n)${NC}"
    read -r install_gum
    if [[ "$install_gum" == "y" || "$install_gum" == "Y" ]]; then
      echo -e "${YELLOW}Installing Gum CLI...${NC}"
      if [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS
        brew install gum
      elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
        # Linux with apt
        if command -v apt-get &> /dev/null; then
          sudo mkdir -p /etc/apt/keyrings
          curl -fsSL https://repo.charm.sh/apt/gpg.key | sudo gpg --dearmor -o /etc/apt/keyrings/charm.gpg
          echo "deb [signed-by=/etc/apt/keyrings/charm.gpg] https://repo.charm.sh/apt/ * *" | sudo tee /etc/apt/sources.list.d/charm.list
          sudo apt update && sudo apt install gum
        # Linux with dnf
        elif command -v dnf &> /dev/null; then
          echo '[charm]
name=Charm
baseurl=https://repo.charm.sh/yum/
enabled=1
gpgcheck=1
gpgkey=https://repo.charm.sh/yum/gpg.key' | sudo tee /etc/yum.repos.d/charm.repo
          sudo dnf install gum
        else
          echo -e "${RED}Unsupported Linux distribution. Please install gum manually: https://github.com/charmbracelet/gum${NC}"
          return 1
        fi
      else
        echo -e "${RED}Unsupported operating system. Please install gum manually: https://github.com/charmbracelet/gum${NC}"
        return 1
      fi

      if command -v gum &> /dev/null; then
        echo -e "${GREEN}Gum CLI installed successfully!${NC}"
        return 0
      else
        echo -e "${RED}Failed to install Gum CLI. Continuing with standard output.${NC}"
        return 1
      fi
    else
      echo -e "${YELLOW}Continuing without Gum CLI.${NC}"
      return 1
    fi
  fi
  return 0
}

# Function to print styled text with gum if available
print_styled() {
  local style=$1
  local message=$2

  if command -v gum &> /dev/null; then
    case $style in
      "info")
        gum style --foreground 33 "$message"
        ;;
      "success")
        gum style --foreground 76 "$message"
        ;;
      "warning")
        gum style --foreground 178 "$message"
        ;;
      "error")
        gum style --foreground 196 "$message"
        ;;
      "header")
        gum style --border normal --margin "1" --padding "1 2" --border-foreground 39 "$message"
        ;;
      *)
        gum style "$message"
        ;;
    esac
  else
    case $style in
      "info")
        echo -e "${YELLOW}$message${NC}"
        ;;
      "success")
        echo -e "${GREEN}$message${NC}"
        ;;
      "warning")
        echo -e "${YELLOW}$message${NC}"
        ;;
      "error")
        echo -e "${RED}$message${NC}"
        ;;
      "header")
        echo -e "\n${GREEN}=== $message ===${NC}\n"
        ;;
      *)
        echo -e "$message"
        ;;
    esac
  fi
}

# Function to confirm with gum if available
confirm() {
  local message=$1
  local default=${2:-false}

  if command -v gum &> /dev/null; then
    if gum confirm --default="$default" "$message"; then
      return 0
    else
      return 1
    fi
  else
    read -p "$message (y/n): " response
    if [[ "$response" == "y" || "$response" == "Y" ]]; then
      return 0
    else
      return 1
    fi
  fi
}

# Function to show a spinner with gum if available
show_spinner() {
  local message=$1
  local command=$2

  if command -v gum &> /dev/null; then
    gum spin --spinner dot --title "$message" -- bash -c "$command"
  else
    echo -e "${YELLOW}$message${NC}"
    bash -c "$command"
  fi
}

# Check if gum is installed
check_and_install_gum

# Parse command line arguments
COMMAND="up"
DETACHED=false
FORCE_REBUILD=false
PRUNE=false

while [[ "$#" -gt 0 ]]; do
    case $1 in
        up|down|restart|logs|status) COMMAND=$1 ;;
        --detached|-d) DETACHED=true ;;
        --rebuild) FORCE_REBUILD=true ;;
        --prune) PRUNE=true ;;
        --help|-h) 
            print_styled "header" "Lyn AI Assistant Docker Development Environment"
            echo "Usage: ./docker-dev.sh [command] [options]"
            echo ""
            echo "Commands:"
            echo "  up        Start all services (default)"
            echo "  down      Stop all services"
            echo "  restart   Restart all services"
            echo "  logs      Show logs for all services"
            echo "  status    Show status of all services"
            echo ""
            echo "Options:"
            echo "  --detached, -d   Run in detached mode"
            echo "  --rebuild        Force rebuild of all images"
            echo "  --prune          Remove all unused containers, networks, and volumes"
            echo "  --help, -h       Show this help message"
            exit 0
            ;;
        *) print_styled "error" "Unknown parameter: $1"; exit 1 ;;
    esac
    shift
done

print_styled "header" "Lyn AI Assistant Docker Development Environment"

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    print_styled "error" "Docker is not running. Please start Docker and try again."
    exit 1
fi

# Function to check if docker-compose is available
check_docker_compose() {
    if command -v docker-compose &> /dev/null; then
        DOCKER_COMPOSE="docker-compose"
    elif docker compose version &> /dev/null; then
        DOCKER_COMPOSE="docker compose"
    else
        print_styled "error" "Docker Compose is not available. Please install Docker Compose and try again."
        exit 1
    fi
}

# Check if docker-compose is available
check_docker_compose

case $COMMAND in
    up)
        print_styled "header" "Starting Services"
        
        if [ "$FORCE_REBUILD" = true ]; then
            print_styled "info" "Forcing rebuild of all images..."
            show_spinner "Rebuilding images..." "$DOCKER_COMPOSE build --no-cache"
        fi
        
        if [ "$DETACHED" = true ]; then
            print_styled "info" "Starting services in detached mode..."
            show_spinner "Starting services..." "$DOCKER_COMPOSE up -d"
        else
            print_styled "info" "Starting services..."
            $DOCKER_COMPOSE up
        fi
        ;;
        
    down)
        print_styled "header" "Stopping Services"
        
        if [ "$PRUNE" = true ]; then
            if confirm "Are you sure you want to stop all services and remove all unused containers, networks, and volumes?"; then
                print_styled "warning" "Stopping services and pruning unused resources..."
                show_spinner "Stopping services..." "$DOCKER_COMPOSE down"
                show_spinner "Pruning unused resources..." "docker system prune -f"
                print_styled "success" "All services stopped and unused resources pruned"
            else
                print_styled "info" "Operation cancelled"
                exit 0
            fi
        else
            print_styled "info" "Stopping services..."
            show_spinner "Stopping services..." "$DOCKER_COMPOSE down"
            print_styled "success" "All services stopped"
        fi
        ;;
        
    restart)
        print_styled "header" "Restarting Services"
        
        print_styled "info" "Stopping services..."
        show_spinner "Stopping services..." "$DOCKER_COMPOSE down"
        
        if [ "$FORCE_REBUILD" = true ]; then
            print_styled "info" "Forcing rebuild of all images..."
            show_spinner "Rebuilding images..." "$DOCKER_COMPOSE build --no-cache"
        fi
        
        print_styled "info" "Starting services..."
        if [ "$DETACHED" = true ]; then
            show_spinner "Starting services..." "$DOCKER_COMPOSE up -d"
        else
            $DOCKER_COMPOSE up
        fi
        ;;
        
    logs)
        print_styled "header" "Service Logs"
        
        print_styled "info" "Showing logs for all services..."
        $DOCKER_COMPOSE logs -f
        ;;
        
    status)
        print_styled "header" "Service Status"
        
        print_styled "info" "Checking service status..."
        $DOCKER_COMPOSE ps
        ;;
esac

# If we're in detached mode and services are up, show the URLs
if [ "$DETACHED" = true ] && [ "$COMMAND" = "up" -o "$COMMAND" = "restart" ]; then
    print_styled "success" "Services started in detached mode"
    
    if command -v gum &> /dev/null; then
        gum style --border normal --margin "1" --padding "1 2" --border-foreground 76 "$(gum join --vertical \
            "$(gum style --foreground 76 "Frontend: http://localhost:5173")" \
            "$(gum style --foreground 76 "Backend API: http://localhost:8080")" \
            "$(gum style --foreground 76 "Embeddings API: http://localhost:8082")")"
    else
        print_styled "success" "Frontend should be available at: http://localhost:5173"
        print_styled "success" "Backend API should be available at: http://localhost:8080"
        print_styled "success" "Embeddings API should be available at: http://localhost:8082"
    fi
    
    print_styled "info" "To view logs, run: ./docker-dev.sh logs"
    print_styled "info" "To stop services, run: ./docker-dev.sh down"
fi
