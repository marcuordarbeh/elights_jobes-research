-- /home/inno/elights_jobes-research/database/migrations/YYYY-MM-DD-HHMMSS_create_initial_schema/down.sql
-- Reverses the changes made in the corresponding up.sql file.

-- Drop triggers first to avoid dependency issues
DROP TRIGGER IF EXISTS set_timestamp_transactions ON core_schema.transactions;
DROP TRIGGER IF EXISTS set_timestamp_wallets ON core_schema.wallets;
DROP TRIGGER IF EXISTS set_timestamp_users ON core_schema.users;

-- Drop the trigger function
DROP FUNCTION IF EXISTS core_schema.trigger_set_timestamp();

-- Drop tables in reverse order of creation (or based on dependencies)
DROP TABLE IF EXISTS core_schema.audit_logs;
DROP TABLE IF EXISTS core_schema.transactions;
DROP TABLE IF EXISTS core_schema.wallets;
DROP TABLE IF EXISTS core_schema.users;

-- Drop schema if needed (use with caution!)
-- DROP SCHEMA IF EXISTS core_schema CASCADE;

-- Drop extensions if they are no longer needed by any part of the DB
-- DROP EXTENSION IF EXISTS pgcrypto;
-- DROP EXTENSION IF EXISTS "uuid-ossp";