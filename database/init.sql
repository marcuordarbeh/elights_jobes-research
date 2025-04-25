-- Create user and database
CREATE USER core_user WITH PASSWORD 'securepassword';
CREATE DATABASE core_db OWNER core_user;

-- Connect to the database and create schema
\c core_db
CREATE SCHEMA IF NOT EXISTS core_schema AUTHORIZATION core_user;

-- Create tables
CREATE TABLE core_schema.users (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL
);

CREATE TABLE core_schema.accounts (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES core_schema.users(id),
    account_number TEXT NOT NULL,
    routing_number TEXT NOT NULL,
    bank_name TEXT NOT NULL
);

CREATE TABLE core_schema.transactions (
    id SERIAL PRIMARY KEY,
    account_id INTEGER REFERENCES core_schema.accounts(id),
    amount NUMERIC NOT NULL,
    currency TEXT NOT NULL,
    transaction_type TEXT NOT NULL,
    status TEXT NOT NULL
);

CREATE TABLE bank_accounts (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  account_number VARCHAR(20) UNIQUE NOT NULL,
  routing_number VARCHAR(9),
  currency VARCHAR(3) CHECK (currency IN ('USD', 'EUR')),
  balance DECIMAL(18, 2) DEFAULT 0,
  created_at TIMESTAMP DEFAULT now()
);

CREATE TABLE transactions (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  account_id UUID REFERENCES bank_accounts(id),
  tx_type VARCHAR(10) CHECK (tx_type IN ('ACH', 'WIRE', 'CARD')),
  amount DECIMAL(18, 2),
  direction VARCHAR(5) CHECK (direction IN ('IN', 'OUT')),
  status VARCHAR(10) DEFAULT 'PENDING',
  metadata JSONB,
  created_at TIMESTAMP DEFAULT now()
);
