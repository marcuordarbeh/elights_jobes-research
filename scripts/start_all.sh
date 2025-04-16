#!/bin/bash

set -e

echo "Starting services with Docker Compose..."
docker-compose up -d

echo "All services started."
