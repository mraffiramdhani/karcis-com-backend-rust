-- Add migration script here
CREATE TABLE roles (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255)
);

INSERT INTO roles (id, name) VALUES (1,'admin'),(2,'customer'),(3,'tenant');