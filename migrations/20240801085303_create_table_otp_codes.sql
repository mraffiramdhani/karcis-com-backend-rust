-- Add migration script here
CREATE TABLE otp_codes (
  id BIGSERIAL PRIMARY KEY,
  code VARCHAR(255),
  is_active BOOLEAN NOT NULL DEFAULT '1',
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  expired_at TIMESTAMP NOT NULL
);