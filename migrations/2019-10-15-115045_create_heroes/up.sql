-- Your SQL goes here

CREATE TABLE IF NOT EXISTS heroes(
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    identity VARCHAR NOT NULL,
    hometown VARCHAR NOT NULL,
    age INT NOT NULL
)