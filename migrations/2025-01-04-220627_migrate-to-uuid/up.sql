CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

BEGIN;

-- Создайте новую таблицу с типом UUID для ролей
CREATE TABLE new_roles
(
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name        VARCHAR NOT NULL
);

-- Создайте временную таблицу для соответствия старых и новых идентификаторов ролей
CREATE TEMP TABLE temp_roles AS
SELECT id AS old_id, uuid_generate_v4() AS new_id FROM roles;

-- Вставьте данные из временной таблицы в новую таблицу ролей
INSERT INTO new_roles (id, name)
SELECT new_id, name FROM roles
                             JOIN temp_roles ON roles.id = temp_roles.old_id;

-- Создайте новую таблицу с типом UUID для пользователей без полей is_married и age
CREATE TABLE new_users
(
    id            UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name          VARCHAR NOT NULL,
    role_id       UUID,
    password_hash VARCHAR NOT NULL
);

-- Переносите данные из старой таблицы в новую, используя временную таблицу для role_id
CREATE TEMP TABLE temp_users AS
SELECT uuid_generate_v4() AS new_id, name, role_id, password_hash FROM users;

INSERT INTO new_users (id, name, role_id, password_hash)
SELECT t.new_id, t.name, tr.new_id, t.password_hash
FROM temp_users t
         JOIN temp_roles tr ON t.role_id = tr.old_id;

-- Удалите старые таблицы
DROP TABLE users;
DROP TABLE roles;

-- Переименуйте новые таблицы
ALTER TABLE new_users RENAME TO users;
ALTER TABLE new_roles RENAME TO roles;

COMMIT;
