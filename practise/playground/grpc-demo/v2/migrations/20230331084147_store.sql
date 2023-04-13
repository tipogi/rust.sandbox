-- Add migration script here
CREATE TABLE IF NOT EXISTS items
(
    sku         TEXT PRIMARY KEY NOT NULL,
    price       REAL NOT NULL,
    quantity    INT NOT NULL,
    name        VARCHAR(50) NOT NULL,
    description VARCHAR(150) NOT NULL
);