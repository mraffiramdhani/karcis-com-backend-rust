-- Add migration script here
CREATE TABLE orders (
  id BIGSERIAL PRIMARY KEY,
  user_id BIGINT NOT NULL,
  hotel_id BIGINT NOT NULL,
  room_type_id BIGINT NOT NULL,
  room_count INTEGER NOT NULL DEFAULT 1,
  guest_count INTEGER NOT NULL DEFAULT 1,
  total_price DECIMAL(12,2) NOT NULL,
  check_in_date TIMESTAMP NOT NULL,
  check_out_date TIMESTAMP DEFAULT NULL,
  duration_in_days INTEGER DEFAULT 1,
  message TEXT,
  is_canceled BOOLEAN DEFAULT '0',
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE NO ACTION ON UPDATE NO ACTION,
  FOREIGN KEY (hotel_id) REFERENCES hotels(id) ON DELETE NO ACTION ON UPDATE NO ACTION,
  FOREIGN KEY (room_type_id) REFERENCES room_types(id) ON DELETE NO ACTION ON UPDATE NO ACTION
)