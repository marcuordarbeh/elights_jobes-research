#!/bin/bash
# /home/inno/elights_jobes-research/scripts/setup.sh
# Setup script for the Elights Jobes Research project.
# Initializes environment, database, migrations, and optionally TLS certs.

set -e # Exit immediately if a command exits with a non-zero status.
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo "--- Elights Jobes Research Setup ---"
cd "$PROJECT_ROOT" # Ensure running from project root

# --- 1. Check Prerequisites ---
echo "[1/6] Checking prerequisites..."
if ! command -v docker &> /dev/null; then echo "ERROR: Docker not found. Please install Docker."; exit 1; fi
DOCKER_COMPOSE_CMD=""
if command -v docker-compose &> /dev/null; then DOCKER_COMPOSE_CMD="docker-compose";
elif docker compose version &> /dev/null; then DOCKER_COMPOSE_CMD="docker compose";
else echo "ERROR: Docker Compose not found. Please install Docker Compose."; exit 1; fi
echo "INFO: Using '$DOCKER_COMPOSE_CMD' for Docker Compose."

if ! command -v cargo &> /dev/null; then echo "ERROR: cargo (Rust toolchain) not found. Please install Rust: https://www.rust-lang.org/tools/install"; exit 1; fi
if ! command -v openssl &> /dev/null; then echo "WARN: openssl not found. Cannot generate self-signed TLS certs if needed."; fi

# Check/Install diesel_cli
if ! command -v diesel &> /dev/null; then
    echo "WARN: diesel-cli not found. Attempting to install (requires cargo)..."
    if cargo install diesel_cli --no-default-features --features postgres; then
        echo "INFO: diesel-cli installed. Ensure ~/.cargo/bin is in your PATH."
        if ! command -v diesel &> /dev/null; then echo "ERROR: diesel-cli installed but not found in PATH. Cannot run migrations."; exit 1; fi
    else
        echo "ERROR: Failed to install diesel-cli. Cannot run migrations."; exit 1;
    fi
else
     echo "INFO: diesel-cli found."
fi
echo "Prerequisites check done."

# --- 2. Environment File Setup ---
echo "[2/6] Setting up environment file..."
if [ ! -f ".env" ]; then
  echo "INFO: .env file not found. Copying from scripts/env.example..."
  if [ -f "scripts/env.example" ]; then
    cp scripts/env.example .env
    echo "SUCCESS: .env file created. IMPORTANT: Please review and update .env with your actual secrets and configurations!"
  else
    echo "ERROR: scripts/env.example not found. Cannot create .env file. Please create it manually."
    exit 1
  fi
else
  echo "INFO: .env file already exists. Ensure it is configured correctly."
fi
# Load .env variables into the current shell for subsequent commands
set -o allexport # Export all variables defined from now on
source .env
set +o allexport # Stop exporting variables
echo "Environment setup done."

# --- 3. Start Database Container ---
echo "[3/6] Starting database container..."
$DOCKER_COMPOSE_CMD up -d postgres
echo "INFO: Waiting for PostgreSQL container to be ready..."
# Improved readiness check loop
MAX_WAIT=30
CURRENT_WAIT=0
while ! $DOCKER_COMPOSE_CMD exec -T postgres pg_isready -U "$POSTGRES_USER" -d "$POSTGRES_DB" -q; do
  if [ $CURRENT_WAIT -ge $MAX_WAIT ]; then
    echo "ERROR: PostgreSQL did not become ready after ${MAX_WAIT} seconds."
    $DOCKER_COMPOSE_CMD logs postgres # Show logs on failure
    exit 1
  fi
  echo "INFO: Database not ready yet, waiting... (${CURRENT_WAIT}s / ${MAX_WAIT}s)"
  sleep 5
  CURRENT_WAIT=$((CURRENT_WAIT + 5))
done
echo "SUCCESS: PostgreSQL container is ready."

# --- 4. Database Setup & Migrations ---
echo "[4/6] Setting up database schema and running migrations..."
# Check if database exists (diesel setup handles this)
echo "INFO: Running 'diesel setup' (creates DB if needed, runs migrations)..."
if diesel setup --database-url "$DATABASE_URL"; then
    echo "SUCCESS: Database setup and migrations completed."
else
    echo "ERROR: Failed to run 'diesel setup'. Check DATABASE_URL and DB container logs."
    exit 1
fi

# --- 5. TLS Certificate Generation (Optional, for Dev) ---
echo "[5/6] Checking TLS certificate generation..."
# Check if TLS is enabled and paths are set in .env
if [[ "${TLS_ENABLED}" == "true" && -n "${TLS_CERT_PATH}" && -n "${TLS_KEY_PATH}" ]]; then
    CERT_FILE="${PROJECT_ROOT}/${TLS_CERT_PATH}"
    KEY_FILE="${PROJECT_ROOT}/${TLS_KEY_PATH}"
    CERT_DIR=$(dirname "$CERT_FILE")

    if [[ -f "$CERT_FILE" && -f "$KEY_FILE" ]]; then
        echo "INFO: TLS certificate and key files already exist at specified paths. Skipping generation."
    elif ! command -v openssl &> /dev/null; then
        echo "WARN: TLS enabled but openssl not found. Skipping self-signed certificate generation."
        echo "WARN: Please generate TLS certificate and key manually and place them at:"
        echo "WARN: Cert: ${CERT_FILE}"
        echo "WARN: Key:  ${KEY_FILE}"
    else
        echo "INFO: TLS enabled, generating self-signed certificate for development..."
        mkdir -p "$CERT_DIR"
        openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
            -keyout "$KEY_FILE" \
            -out "$CERT_FILE" \
            -subj "/C=US/ST=Dev/L=Local/O=ElightsDev/CN=localhost" # Basic subject
        echo "SUCCESS: Generated self-signed TLS certificate and key."
        echo "         Cert: ${CERT_FILE}"
        echo "         Key:  ${KEY_FILE}"
        echo "WARNING: These are self-signed certificates suitable ONLY for development/testing."
    fi
else
    echo "INFO: TLS not enabled or paths not set in .env. Skipping certificate generation."
fi

# --- 6. Firewall Rule (Example - Commented Out) ---
echo "[6/6] Firewall configuration check..."
# This requires sudo and specific firewall knowledge (ufw example)
# Generally better handled manually or via Infrastructure as Code (IaC)
# Uncomment and adapt if needed for specific development environments.
# if grep -q "FT_ASSET_ALLOWED_IPS" .env && grep -q "BANK_PORT" .env; then
#     FT_ASSET_IP=$(grep FT_ASSET_ALLOWED_IPS .env | cut -d '=' -f2 | cut -d ',' -f1 | tr -d '[:space:]')
#     BANK_PORT_VAL=$(grep BANK_PORT .env | cut -d '=' -f2 | tr -d '[:space:]')
#     if [[ -n "$FT_ASSET_IP" && -n "$BANK_PORT_VAL" ]]; then
#         if command -v ufw &> /dev/null; then
#             echo "INFO: Attempting to configure UFW firewall (requires sudo)..."
#             echo "INFO: Allow traffic from $FT_ASSET_IP to port $BANK_PORT_VAL"
#             # Use 'ufw status | grep ...' to check if rule exists before inserting?
#             if sudo ufw status | grep -q "ALLOW IN       Anywhere from ${FT_ASSET_IP} to any port ${BANK_PORT_VAL}"; then
#                  echo "INFO: UFW rule already exists or similar rule present."
#             else
#                  sudo ufw allow from "$FT_ASSET_IP" to any port "$BANK_PORT_VAL" proto tcp comment "Allow FT Asset Mgmt"
#                  sudo ufw reload
#                  echo "INFO: UFW rule potentially added/updated."
#             fi
#         else
#             echo "WARN: ufw command not found. Skipping firewall configuration."
#         fi
#     else
#         echo "WARN: FT_ASSET_ALLOWED_IPS or BANK_PORT not defined correctly in .env. Skipping firewall rule."
#     fi
# else
#     echo "INFO: FT_ASSET_ALLOWED_IPS or BANK_PORT not found in .env. Skipping firewall rule."
# fi
echo "INFO: Firewall check complete (example rule commented out)."


echo ""
echo "--- Setup Complete ---"
echo "Next Steps:"
echo "1. If you haven't already, **carefully review and update the .env file** with your real secrets and configuration."
echo "2. Build and start all services using: ./scripts/start_all.sh"
echo "---"