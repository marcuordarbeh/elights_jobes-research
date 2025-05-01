#!/bin/bash
# /home/inno/elights_jobes-research/scripts/start_all.sh
# Starts all services defined in docker-compose.yml in detached mode.

set -e

# Determine docker-compose command
if ! command -v docker-compose &> /dev/null; then
     if ! docker compose version &> /dev/null; then
        echo "ERROR: Docker Compose (v1 or v2) could not be found. Please install Docker Compose."
        exit 1
     fi
     DOCKER_COMPOSE_CMD="docker compose"
else
     DOCKER_COMPOSE_CMD="docker-compose"
fi


echo "INFO: Starting all services using '$DOCKER_COMPOSE_CMD'..."
# Use --build flag to ensure images are rebuilt if Dockerfiles changed
$DOCKER_COMPOSE_CMD up --build -d

echo "INFO: All services started in detached mode."
echo "Use '$DOCKER_COMPOSE_CMD logs -f' to view logs."
echo "Use '$DOCKER_COMPOSE_CMD down' to stop services."