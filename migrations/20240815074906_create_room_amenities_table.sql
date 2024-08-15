-- Add migration script here
CREATE TABLE room_amenities (
  id BIGSERIAL PRIMARY KEY,
  amenity_id BIGINT NOT NULL,
  hotel_id BIGINT NOT NULL,
  room_type_id BIGINT NOT NULL,
  price DECIMAL(12,2) NOT NULL,
  FOREIGN KEY (amenity_id) REFERENCES amenities(id) ON DELETE CASCADE ON UPDATE CASCADE,
  FOREIGN KEY (hotel_id) REFERENCES hotels(id) ON DELETE CASCADE ON UPDATE CASCADE,
  FOREIGN KEY (room_type_id) REFERENCES room_types(id) ON DELETE CASCADE ON UPDATE CASCADE
)