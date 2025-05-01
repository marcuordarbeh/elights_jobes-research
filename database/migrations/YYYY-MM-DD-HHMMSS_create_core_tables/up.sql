-- /home/inno/elights_jobes-research/database/migrations/YYYY-MM-DD-HHMMSS_create_core_tables/up.sql
-- Adjust timestamp in directory name

-- Users table (using username as primary identifier seems more common than serial id)
CREATE TABLE core_schema.users (
    username TEXT PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL, -- Store hashed passwords only!
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Accounts table (linking fiat/bank accounts to users)
CREATE TABLE core_schema.accounts (
    id SERIAL PRIMARY KEY, -- Or UUID if preferred: UUID PRIMARY KEY DEFAULT uuid_generate_v4()
    owner_username TEXT NOT NULL REFERENCES core_schema.users(username) ON DELETE CASCADE,
    account_identifier TEXT NOT NULL UNIQUE, -- Could be IBAN, account number etc. depending on type
    account_type TEXT NOT NULL, -- e.g., 'FIAT_BANK', 'CRYPTO_XMR', 'CRYPTO_BTC'
    currency TEXT NOT NULL, -- ISO 4217 for fiat, Ticker for crypto (e.g., USD, EUR, XMR, BTC)
    balance NUMERIC(18, 8) NOT NULL DEFAULT 0.00, -- Increased precision for crypto
    bank_name TEXT, -- For FIAT_BANK type
    routing_number TEXT, -- For FIAT_BANK type (USA specific)
    iban TEXT, -- For FIAT_BANK type (Europe specific)
    bic_swift TEXT, -- For FIAT_BANK type (SWIFT code) [cite: 8]
    crypto_address TEXT, -- For CRYPTO types
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
    -- Consider adding status (active, inactive, closed)
);

-- Create index on owner for faster lookups
CREATE INDEX idx_accounts_owner ON core_schema.accounts(owner_username);

-- Transactions table (more detailed)
CREATE TABLE core_schema.transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(), -- Using UUID for transaction IDs [cite: 10886]
    debit_account_id INTEGER REFERENCES core_schema.accounts(id), -- Nullable for deposits/external source
    credit_account_id INTEGER REFERENCES core_schema.accounts(id), -- Nullable for withdrawals/external dest
    amount NUMERIC(18, 8) NOT NULL, -- Amount transferred
    currency TEXT NOT NULL, -- Currency of the transaction
    transaction_type TEXT NOT NULL, -- e.g., 'ACH', 'WIRE', 'CRYPTO_BTC', 'CRYPTO_XMR', 'INTERNAL_TRANSFER', 'CARD' [cite: 10886]
    status TEXT NOT NULL DEFAULT 'PENDING', -- e.g., PENDING, COMPLETED, FAILED, CANCELLED
    description TEXT, -- Optional description
    metadata JSONB, -- Store type-specific details (e.g., swift details, crypto tx hash, ACH trace id) [cite: 10886]
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
    -- Consider adding foreign keys to users if direct user association is needed beyond account owner
);

-- Indexes for common queries
CREATE INDEX idx_transactions_debit_account ON core_schema.transactions(debit_account_id);
CREATE INDEX idx_transactions_credit_account ON core_schema.transactions(credit_account_id);
CREATE INDEX idx_transactions_status ON core_schema.transactions(status);
CREATE INDEX idx_transactions_type ON core_schema.transactions(transaction_type);
CREATE INDEX idx_transactions_created_at ON core_schema.transactions(created_at DESC);

-- Add trigger function/procedure for updated_at timestamp columns if needed
-- CREATE OR REPLACE FUNCTION update_modified_column()
-- RETURNS TRIGGER AS $$
-- BEGIN
--     NEW.updated_at = now();
--     RETURN NEW;
-- END;
-- $$ language 'plpgsql';

-- CREATE TRIGGER update_accounts_modtime BEFORE UPDATE ON core_schema.accounts FOR EACH ROW EXECUTE PROCEDURE update_modified_column();
-- CREATE TRIGGER update_transactions_modtime BEFORE UPDATE ON core_schema.transactions FOR EACH ROW EXECUTE PROCEDURE update_modified_column();