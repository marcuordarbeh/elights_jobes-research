-- /home/inno/elights_jobes-research/database/init.sql

-- Best practice: Use environment variables in production instead of hardcoding user/password.
-- This script is often run automatically by the postgres docker image.
-- CREATE USER core_user WITH PASSWORD 'securepassword'; -- User created by Docker env vars
-- CREATE DATABASE core_db OWNER core_user; -- DB created by Docker env vars

-- Connect to the database (psql command, not needed in init script run by docker)
-- \c core_db

-- Create the schema if it doesn't exist and set owner
CREATE SCHEMA IF NOT EXISTS core_schema AUTHORIZATION ${POSTGRES_USER:-core_user};

-- Optional: Set default search path for the user
-- ALTER ROLE ${POSTGRES_USER:-core_user} SET search_path TO core_schema, public;

-- Grant usage to the schema owner
GRANT USAGE ON SCHEMA core_schema TO ${POSTGRES_USER:-core_user};

-- Note: Table creation is handled by migrations (up.sql files)
-- Ensure necessary extensions like pgcrypto or uuid-ossp are enabled if needed
-- CREATE EXTENSION IF NOT EXISTS "uuid-ossp"; -- If using UUIDs [cite: 10885]
-- CREATE EXTENSION IF NOT EXISTS pgcrypto; -- If using pgcrypto functions [cite: 13851]