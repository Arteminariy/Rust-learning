CREATE TABLE roles
(
    id   SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL
);

ALTER TABLE users
    ADD COLUMN role_id INT REFERENCES roles (id);
ALTER TABLE users
    ADD COLUMN password_hash VARCHAR NOT NULL;
