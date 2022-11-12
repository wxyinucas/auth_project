INSERT INTO auth.users (email, password)
VALUES ('rex@mail.com', 'rex')
RETURNING id;
INSERT INTO auth.users (email, password)
VALUES ('miao@mail.com', 'miao')
RETURNING id;
