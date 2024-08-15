-- Add migration script here
CREATE TABLE hotel_amenities (
  amenities_id BIGINT,
  hotel_id BIGINT,
  FOREIGN KEY (amenities_id) REFERENCES amenities(id) ON DELETE CASCADE ON UPDATE CASCADE,
  FOREIGN KEY (hotel_id) REFERENCES hotels(id) ON DELETE CASCADE ON UPDATE CASCADE
)