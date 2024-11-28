ALTER TABLE users DROP COLUMN role_id;
ALTER TABLE users DROP COLUMN password_hash;

DROP TABLE roles;
