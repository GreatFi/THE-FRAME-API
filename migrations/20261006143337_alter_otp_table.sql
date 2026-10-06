-- Add migration script here
ALTER TABLE otp ADD COLUMN attempts INT NOT NULL DEFAULT 0;