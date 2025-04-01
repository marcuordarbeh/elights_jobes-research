#!/bin/bash
# setup.sh - Installs required packages and configures PostgreSQL, Tor, and Redis.
if [ "$(id -u)" -ne 0 ]; then
  echo "This script must be run as root. Try 'sudo ./scripts/setup.sh'"
  exit 1
fi

echo "Updating packages..."
apt update

echo "Installing Tor, PostgreSQL, and Redis..."
apt install -y tor postgresql postgresql-contrib redis-server

echo "Starting PostgreSQL service..."
service postgresql start

echo "Setting up PostgreSQL database and user..."
sudo -u postgres psql -c "CREATE DATABASE payment_system;"
sudo -u postgres psql -c "CREATE USER payment_user WITH ENCRYPTED PASSWORD 'password';"
sudo -u postgres psql -c "GRANT ALL PRIVILEGES ON DATABASE payment_system TO payment_user;"

if [ -f ./database/init.sql ]; then
  echo "Initializing database schema..."
  sudo -u postgres psql -d payment_system -f ./database/init.sql
else
  echo "Warning: Database initialization script not found."
fi

echo "Starting Tor service..."
service tor start

echo "Starting Redis service..."
service redis-server start

echo "Writing environment variables to project root .env file..."
cat > ../.env <<EOL
DATABASE_URL=postgres://payment_user:password@localhost:5432/payment_system
STRIPE_SECRET_KEY=your_stripe_secret_key_here
JWT_SECRET=your_jwt_secret_here
REDIS_URL=redis://127.0.0.1:6379/0
PLAID_API_KEY=client_id_123456789
NACHA_API_KEY=
EOL

echo "Setup complete. PostgreSQL, Tor, and Redis are running."
