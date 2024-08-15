-- Add migration script here
CREATE TABLE hotel_rooms (
  id BIGSERIAL PRIMARY KEY,
  room_type_id BIGINT NOT NULL,
  hotel_id BIGINT NOT NULL,
  price DECIMAL(12,2) NOT NULL,
  max_capacity INTEGER NOT NULL DEFAULT 1,
  description TEXT NOT NULL,
  FOREIGN KEY (room_type_id) REFERENCES room_types(id) ON DELETE CASCADE ON UPDATE CASCADE,
  FOREIGN KEY (hotel_id) REFERENCES hotels(id) ON DELETE CASCADE ON UPDATE CASCADE
)