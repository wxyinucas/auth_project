INSERT INTO auth.users (name, email, password) VALUES ('rex', 'rex@example.com', 'rex') RETURNING id;
INSERT INTO auth.users (name, email, password, auth_level) VALUES ('miao', 'miao@example.com', 'miao', 'customer') RETURNING id;
