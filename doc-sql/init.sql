CREATE SCHEMA auth;
CREATE TYPE auth.user_auth_level AS ENUM ('admin', 'customer');
CREATE TYPE auth.user_status AS ENUM ('active', 'freeze', 'dropped');

CREATE TABLE auth.users
(
    id         SERIAL PRIMARY KEY,
    name       VARCHAR(255)    NOT NULL,
    email      VARCHAR(255)    Not Null,
    password   VARCHAR(255)    NOT NULL,
    auth_level auth.user_auth_level NOT NULL DEFAULT 'admin',
    status     auth.user_status     NOT NULL DEFAULT 'active'
)
