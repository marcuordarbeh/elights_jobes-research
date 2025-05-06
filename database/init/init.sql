-- /home/inno/elights_jobes-research/database/init/init-db.sql
-- This script is run automatically by the PostgreSQL container on first start.

-- Create extensions required by the application (if any)
-- Example: For UUID generation or crypto functions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- Create the main schema if it doesn't exist
-- Note: User/DB are created by Docker Compose environment variables
DO $$
BEGIN
   IF NOT EXISTS (
      SELECT FROM pg_catalog.pg_namespace
      WHERE nspname = 'core_schema') THEN

      CREATE SCHEMA core_schema;
      RAISE NOTICE 'Schema core_schema created.';
   ELSE
      RAISE NOTICE 'Schema core_schema already exists.';
   END IF;
END
$$;

-- Grant usage on the schema to the application user
-- The user is defined by POSTGRES_USER env var
GRANT USAGE ON SCHEMA core_schema TO "${POSTGRES_USER}";
-- Grant create permissions if migrations are run by this user
GRANT CREATE ON SCHEMA core_schema TO "${POSTGRES_USER}";
-- Set default search path for the user (optional, can be set in connection string)
-- ALTER ROLE "${POSTGRES_USER}" SET search_path TO core_schema, public;

-- Note: Table creation and modifications should be handled by migrations (`diesel migration run`).
-- This script is primarily for one-time setup like extensions or schemas.