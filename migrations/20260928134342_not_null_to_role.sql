-- Add migration script here
ALTER TABLE users
ALTER COLUMN role SET NOT NULL;