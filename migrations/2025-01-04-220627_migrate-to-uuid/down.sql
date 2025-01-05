BEGIN;

DROP TABLE users;
DROP TABLE roles;

CREATE TABLE users
(
    id           SERIAL PRIMARY KEY,
    name         VARCHAR NOT NULL,
    age          int4    NOT NULL,
    is_married   BOOLEAN NOT NULL DEFAULT FALSE,
    role_id      INT REFERENCES roles (id),
    password_hash VARCHAR NOT NULL
);

CREATE TABLE roles
(
    id   SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL
);

INSERT INTO users (id, name, age, is_married, role_id, password_hash)
SELECT nextval('users_id_seq'), name, age, is_married, role_id, password_hash FROM new_users;

INSERT INTO roles (id, name)
SELECT nextval('roles_id_seq'), name FROM new_roles;

COMMIT;
