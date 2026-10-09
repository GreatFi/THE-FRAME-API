-- Add migration script here
CREATE INDEX idx_otp_user_live ON otp (user_id, created_at DESC);