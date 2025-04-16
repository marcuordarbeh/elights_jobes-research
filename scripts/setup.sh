#!/bin/bash

set -e

echo "Initializing database..."
psql -U postgres -f database/init.sql

echo "Running migrations..."
diesel migration run

echo "Setting up environment..."
cp scripts/env.example .env

echo "Setup complete."

# #!/bin/bash
# # setup.sh - Installs required packages and configures PostgreSQL, Tor, and Redis.

# echo "Updating Homebrew..."
# brew update

# echo "Installing PostgreSQL and Redis..."
# brew install postgresql redis

# echo "Starting PostgreSQL and Redis..."
# brew services start postgresql
# brew services start redis

# redis = { version = "0.23", features = ["async-std-comp"] }
# psql postgres -tc "SELECT 1 FROM pg_roles WHERE rolname='payment_user'" | grep -q 1 || psql postgres -c "CREATE ROLE payment_user WITH LOGIN PASSWORD 'yourpassword';"
# psql postgres -c "ALTER ROLE payment_user CREATEDB;"

# psql postgres -tc "SELECT 1 FROM pg_database WHERE datname='payment_system'" | grep -q 1 || psql postgres -c "CREATE DATABASE payment_system OWNER payment_user;"

# if [ -f ./database/init.sql ]; then
#   echo "Initializing database schema..."
#   sudo -u postgres psql -d payment_system -f ./database/init.sql
# else
#   echo "Warning: Database initialization script not found."
# fi

# echo "Starting Tor service..."
# service tor start

# echo "Starting Redis service..."
# service redis-server start

# echo "Writing environment variables to project root .env file..."
# cat > .env <<EOL
# DATABASE_URL=postgres://payment_user:password@localhost:5432/payment_system
# STRIPE_SECRET_KEY=your_stripe_secret_key_here
# JWT_SECRET=your_jwt_secret_here
# REDIS_URL=redis://127.0.0.1:6379/0
# PLAID_API_KEY=client_id_123456789
# NACHA_API_KEY=
# EOL

# echo "Setup complete. PostgreSQL, Tor, and Redis are running."
