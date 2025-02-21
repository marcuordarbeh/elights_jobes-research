#!/bin/bash

# Install necessary packages
apt update
apt install -y tor postgresql postgresql-contrib

# Start PostgreSQL service
service postgresql start

# Set up PostgreSQL database
sudo -u postgres psql -c "CREATE DATABASE payment_system;"
sudo -u postgres psql -c "CREATE USER payment_user WITH ENCRYPTED PASSWORD 'password';"
sudo -u postgres psql -c "GRANT ALL PRIVILEGES ON DATABASE payment_system TO payment_user;"

# Run database initialization script
sudo -u postgres psql -d payment_system -f /path/to/your/database/init.sql

# Start Tor service
service tor start

# Set up environment variables
echo "DATABASE_URL=postgres://payment_user:password@localhost:5432/payment_system" > /path/to/your/.env
echo "STRIPE_SECRET_KEY=your_stripe_secret_key" >> /path/to/your/.env
echo "JWT_SECRET=your_jwt_secret" >> /path/to/your/.env