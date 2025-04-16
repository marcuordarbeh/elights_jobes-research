CREATE TABLE accounts (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    balance NUMERIC(12, 2) NOT NULL DEFAULT 0.00,
    currency TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);
