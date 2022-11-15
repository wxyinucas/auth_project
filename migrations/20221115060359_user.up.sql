CREATE TYPE auth.user_status AS ENUM ('active', 'freeze');

CREATE TABLE auth.users
(
    id       SERIAL PRIMARY KEY,
    email    VARCHAR(255)     Not Null,
    password VARCHAR(255)     NOT NULL,
    status   auth.user_status NOT NULL DEFAULT 'active'
)
