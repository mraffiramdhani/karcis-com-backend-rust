-- Add migration script here
CREATE TABLE balance_histories (
  id BIGSERIAL PRIMARY KEY,
  user_id BIGINT,
  balance_id BIGINT,
  balance DECIMAL(12, 2),
  top_up DECIMAL(12, 2) DEFAULT 0,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE ON UPDATE CASCADE,
  FOREIGN KEY (balance_id) REFERENCES balances(id) ON DELETE CASCADE ON UPDATE CASCADE
);