-- Add migration script here
CREATE TABLE revoked_token (
  id BIGSERIAL PRIMARY KEY,
  token TEXT NOT NULL,
  is_revoked BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);