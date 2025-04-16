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
