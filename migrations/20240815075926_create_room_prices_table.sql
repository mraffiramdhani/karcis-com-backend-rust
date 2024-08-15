-- Add migration script here
CREATE TABLE room_prices (
  id BIGSERIAL PRIMARY KEY,
  hotel_id BIGINT NOT NULL,
  room_type_id BIGINT NOT NULL,
  price DECIMAL(12,2) NOT NULL,
  price_breakfast_included DECIMAL(12,2) NOT NULL,
  FOREIGN KEY (hotel_id) REFERENCES hotels(id) ON DELETE CASCADE ON UPDATE CASCADE,
  FOREIGN KEY (room_type_id) REFERENCES room_types(id) ON DELETE CASCADE ON UPDATE CASCADE 
)