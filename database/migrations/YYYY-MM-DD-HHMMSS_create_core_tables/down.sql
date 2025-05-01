-- /home/inno/elights_jobes-research/database/migrations/YYYY-MM-DD-HHMMSS_create_core_tables/down.sql
-- Adjust timestamp in directory name

DROP TABLE IF EXISTS core_schema.transactions;
DROP TABLE IF EXISTS core_schema.accounts;
DROP TABLE IF EXISTS core_schema.users;
-- DROP FUNCTION IF EXISTS update_modified_column(); -- Drop trigger function if created