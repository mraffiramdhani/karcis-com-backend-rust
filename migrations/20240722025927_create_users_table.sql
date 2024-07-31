-- Add migration script here
CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    first_name VARCHAR(255),
    last_name VARCHAR(255),
    phone VARCHAR(255),
    username VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    title VARCHAR(255),
    image VARCHAR(255)
);