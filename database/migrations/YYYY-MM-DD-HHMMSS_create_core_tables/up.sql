-- /home/inno/elights_jobes-research/database/migrations/YYYY-MM-DD-HHMMSS_create_initial_schema/up.sql

-- Create Users Table
CREATE TABLE core_schema.users (
    user_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(), -- Use UUID for primary key
    username VARCHAR(100) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash TEXT NOT NULL, -- Store securely hashed passwords (e.g., bcrypt)
    -- Add roles, status, 2FA settings etc. as needed
    -- role VARCHAR(50) NOT NULL DEFAULT 'user',
    -- status VARCHAR(20) NOT NULL DEFAULT 'active', -- e.g., active, suspended, pending_verification
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_users_username ON core_schema.users(username);
CREATE INDEX idx_users_email ON core_schema.users(email);

-- Create Wallets Table (for internal tracking of crypto addresses/accounts)
CREATE TABLE core_schema.wallets (
    wallet_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES core_schema.users(user_id) ON DELETE CASCADE,
    wallet_type VARCHAR(50) NOT NULL, -- e.g., 'FIAT_USD', 'CRYPTO_BTC', 'CRYPTO_XMR', 'FIAT_EUR'
    currency_code VARCHAR(10) NOT NULL, -- ISO 4217 (USD, EUR) or Ticker (BTC, XMR)
    balance NUMERIC(38, 18) NOT NULL DEFAULT 0.0, -- High precision for crypto
    -- Fiat specific fields
    bank_name VARCHAR(255),
    account_number_hash TEXT, -- Hash or encrypt sensitive numbers
    iban_hash TEXT,
    bic_swift VARCHAR(11),
    routing_number_hash TEXT,
    -- Crypto specific fields
    address VARCHAR(255), -- Public address
    address_index INTEGER, -- For HD wallets / Monero subaddresses
    -- BlindAE specific (conceptual - needs real implementation)
    -- blindae_encrypted_balance TEXT, -- Encrypted balance using BlindAE
    -- blindae_policy_id VARCHAR(100), -- Reference to the policy used
    status VARCHAR(20) NOT NULL DEFAULT 'active', -- e.g., active, inactive
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_wallets_user_id ON core_schema.wallets(user_id);
CREATE INDEX idx_wallets_type_currency ON core_schema.wallets(wallet_type, currency_code);
CREATE INDEX idx_wallets_address ON core_schema.wallets(address); -- If querying by address

-- Create Transactions Table
CREATE TABLE core_schema.transactions (
    transaction_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    debit_wallet_id UUID REFERENCES core_schema.wallets(wallet_id), -- Nullable for deposits/external sources
    credit_wallet_id UUID REFERENCES core_schema.wallets(wallet_id), -- Nullable for withdrawals/external sinks
    transaction_type VARCHAR(50) NOT NULL, -- e.g., 'ACH_CREDIT', 'WIRE_OUT', 'CARD_DEBIT', 'CRYPTO_BTC_SEND', 'CRYPTO_XMR_RECEIVE', 'INTERNAL_TRANSFER', 'CONVERSION'
    status VARCHAR(30) NOT NULL DEFAULT 'PENDING', -- e.g., PENDING, PROCESSING, REQUIRES_ACTION, COMPLETED, FAILED, CANCELLED, SETTLED (RTGS)
    amount NUMERIC(38, 18) NOT NULL, -- Amount transferred
    currency_code VARCHAR(10) NOT NULL, -- Currency of the transaction
    description TEXT, -- User-provided or generated description
    external_ref_id VARCHAR(255) UNIQUE, -- Reference from external system (e.g., bank API, crypto tx hash, payment gateway ID)
    metadata JSONB, -- Store type-specific details (swift details, crypto tx info, ACH trace, card auth codes, ISO 20022 refs)
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    settlement_at TIMESTAMPTZ -- Timestamp when RTGS/final settlement occurred
);
CREATE INDEX idx_transactions_debit_wallet ON core_schema.transactions(debit_wallet_id);
CREATE INDEX idx_transactions_credit_wallet ON core_schema.transactions(credit_wallet_id);
CREATE INDEX idx_transactions_status ON core_schema.transactions(status);
CREATE INDEX idx_transactions_type ON core_schema.transactions(transaction_type);
CREATE INDEX idx_transactions_external_ref ON core_schema.transactions(external_ref_id);
CREATE INDEX idx_transactions_created_at ON core_schema.transactions(created_at DESC);

-- Create Audit Log Table
CREATE TABLE core_schema.audit_logs (
    log_id BIGSERIAL PRIMARY KEY,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    user_id UUID REFERENCES core_schema.users(user_id), -- Nullable for system events
    actor_identifier VARCHAR(255) NOT NULL, -- Username, System Component, API Key ID
    action VARCHAR(100) NOT NULL, -- e.g., 'LOGIN_SUCCESS', 'PAYMENT_INITIATED', 'CONFIG_UPDATE'
    target_type VARCHAR(100), -- e.g., 'TRANSACTION', 'USER', 'WALLET', 'SYSTEM'
    target_id VARCHAR(255), -- UUID or other identifier of the target entity
    outcome VARCHAR(20) NOT NULL, -- 'SUCCESS', 'FAILURE'
    details JSONB, -- Additional context (e.g., parameters, changes, source IP)
    error_message TEXT -- Store error details on failure
);
CREATE INDEX idx_audit_logs_timestamp ON core_schema.audit_logs(timestamp DESC);
CREATE INDEX idx_audit_logs_user_id ON core_schema.audit_logs(user_id);
CREATE INDEX idx_audit_logs_action ON core_schema.audit_logs(action);
CREATE INDEX idx_audit_logs_target ON core_schema.audit_logs(target_type, target_id);

-- Trigger function to automatically update `updated_at` columns
CREATE OR REPLACE FUNCTION core_schema.trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Apply the trigger to tables with `updated_at`
CREATE TRIGGER set_timestamp_users
BEFORE UPDATE ON core_schema.users
FOR EACH ROW
EXECUTE FUNCTION core_schema.trigger_set_timestamp();

CREATE TRIGGER set_timestamp_wallets
BEFORE UPDATE ON core_schema.wallets
FOR EACH ROW
EXECUTE FUNCTION core_schema.trigger_set_timestamp();

CREATE TRIGGER set_timestamp_transactions
BEFORE UPDATE ON core_schema.transactions
FOR EACH ROW
EXECUTE FUNCTION core_schema.trigger_set_timestamp();