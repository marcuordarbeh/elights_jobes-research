-- init.sql - Database schema for payment system.
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    role VARCHAR(50) NOT NULL
);

CREATE TABLE ach_details (
    id SERIAL PRIMARY KEY,
    details TEXT NOT NULL
);

CREATE TABLE bank_transfers (
    id SERIAL PRIMARY KEY,
    bank_name VARCHAR(255) NOT NULL,
    account_number VARCHAR(255) NOT NULL
);
