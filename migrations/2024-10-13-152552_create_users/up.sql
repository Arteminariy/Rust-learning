CREATE TABLE users
(
    id         SERIAL PRIMARY KEY,
    name       VARCHAR NOT NULL,
    age        int4    NOT NULL,
    is_married BOOLEAN NOT NULL DEFAULT FALSE
)