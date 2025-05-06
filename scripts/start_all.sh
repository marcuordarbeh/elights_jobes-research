#!/bin/bash
# /home/inno/elights_jobes-research/scripts/start_all.sh
# Builds images if necessary and starts all services defined in docker-compose.yml
# in detached mode.

set -e
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_ROOT" # Ensure running from project root

# Determine docker-compose command
DOCKER_COMPOSE_CMD=""
if command -v docker-compose &> /dev/null; then DOCKER_COMPOSE_CMD="docker-compose";
elif docker compose version &> /dev/null; then DOCKER_COMPOSE_CMD="docker compose";
else echo "ERROR: Docker Compose not found. Please install Docker Compose."; exit 1; fi

echo "INFO: Building images and starting all services using '$DOCKER_COMPOSE_CMD'..."
# Use --build flag to ensure images are rebuilt if Dockerfiles changed
# Use -d for detached mode
$DOCKER_COMPOSE_CMD up --build -d

echo ""
echo "--- Services Started ---"
echo "Use '$DOCKER_COMPOSE_CMD ps' to see running containers."
echo "Use '$DOCKER_COMPOSE_CMD logs -f' to view logs."
echo "Use '$DOCKER_COMPOSE_CMD down' to stop all services."
echo "---"