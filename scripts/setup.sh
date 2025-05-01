#!/bin/bash
# /home/inno/elights_jobes-research/scripts/setup.sh
# Basic setup script for the Elights Jobes Research project.

set -e # Exit immediately if a command exits with a non-zero status.

echo "--- Elights Jobes Research Setup ---"

# 1. Environment File Check/Setup
if [ ! -f ".env" ]; then
  echo "INFO: .env file not found. Copying from env.example..."
  if [ -f "scripts/env.example" ]; then
    cp scripts/env.example .env
    echo "INFO: .env file created. Please review and update it with your actual secrets and configurations."
  else
    echo "ERROR: scripts/env.example not found. Cannot create .env file. Please create it manually."
    exit 1
  fi
else
  echo "INFO: .env file already exists."
fi

# Load .env to get database credentials (optional, Docker Compose usually handles this for DB container)
# export $(grep -v '^#' .env | xargs)

# 2. Check Docker & Docker Compose
if ! command -v docker &> /dev/null; then
    echo "ERROR: Docker could not be found. Please install Docker."
    exit 1
fi
if ! command -v docker-compose &> /dev/null; then
     # Check for new 'docker compose' command
     if ! docker compose version &> /dev/null; then
        echo "ERROR: Docker Compose (v1 or v2) could not be found. Please install Docker Compose."
        exit 1
     fi
     DOCKER_COMPOSE_CMD="docker compose"
else
     DOCKER_COMPOSE_CMD="docker-compose"
fi
echo "INFO: Using '$DOCKER_COMPOSE_CMD' for Docker Compose commands."


# 3. Start PostgreSQL (if not already running) to apply migrations
echo "INFO: Starting PostgreSQL container via Docker Compose..."
$DOCKER_COMPOSE_CMD up -d postgres
echo "INFO: Waiting for PostgreSQL to be ready..."
# Simple wait loop - consider a more robust check if needed
sleep 15

# 4. Database Initialization & Migrations
# Check if diesel-cli is installed
if ! command -v diesel &> /dev/null; then
    echo "WARN: diesel-cli not found. Attempting to install..."
    # Check if cargo is installed
    if ! command -v cargo &> /dev/null; then
        echo "ERROR: cargo (Rust toolchain) not found. Cannot install diesel-cli. Please install Rust: https://www.rust-lang.org/tools/install"
        exit 1
    fi
    cargo install diesel_cli --no-default-features --features postgres
    # Advise user to add ~/.cargo/bin to PATH if needed
    echo "INFO: diesel-cli installed. Ensure ~/.cargo/bin is in your PATH."
    # Re-check after install attempt
    if ! command -v diesel &> /dev/null; then
       echo "ERROR: diesel-cli installation failed or not in PATH. Cannot run migrations."
       exit 1
    fi
fi

# Run Diesel migrations
echo "INFO: Running database migrations..."
# Assuming DATABASE_URL is set correctly in the shell environment or .env file for diesel
# Load .env if not already loaded
# export $(grep -v '^#' .env | xargs) # Uncomment if needed
if [ -z "$DATABASE_URL" ]; then
    echo "ERROR: DATABASE_URL not set. Cannot run migrations. Source your .env file or set it."
    exit 1
fi
diesel migration run

echo "INFO: Database migrations complete."

# 5. Optional: Firewall Configuration (Example for ufw)
# This part requires sudo and knowledge of the specific IP/Port
# It's often better handled by infrastructure setup (IaC) or manually.
# Uncomment and adapt if truly needed within this script.
# echo "INFO: Configuring firewall (requires sudo)..."
# FT_ASSET_IP=$(grep FT_ASSET_ALLOWED_IPS .env | cut -d '=' -f2 | cut -d ',' -f1) # Get first IP
# BANK_PORT=$(grep BANK_PORT .env | cut -d '=' -f2)
# if [ -n "$FT_ASSET_IP" ] && [ -n "$BANK_PORT" ]; then
#   if command -v ufw &> /dev/null; then
#       echo "Attempting to allow traffic from $FT_ASSET_IP to port $BANK_PORT..."
#       sudo ufw insert 1 allow from "$FT_ASSET_IP" to any port "$BANK_PORT" proto tcp comment 'Allow FT Asset Management'
#       sudo ufw reload
#       echo "INFO: Firewall rule potentially added/updated for $FT_ASSET_IP -> $BANK_PORT."
#   else
#       echo "WARN: ufw command not found. Skipping firewall configuration."
#   fi
# else
#   echo "WARN: FT_ASSET_ALLOWED_IPS or BANK_PORT not found in .env. Skipping firewall configuration."
# fi


# 6. Build Docker images (optional, 'docker-compose up' will also build)
# echo "INFO: Building Docker images..."
# $DOCKER_COMPOSE_CMD build

echo "--- Setup complete ---"
echo "You may now need to:"
echo "  - Review and update the .env file with your actual secrets."
echo "  - Start all services using: ./scripts/start_all.sh"