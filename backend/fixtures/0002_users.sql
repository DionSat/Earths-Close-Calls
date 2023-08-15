DELETE FROM users;

-- Reset primary key id to 1
SELECT setval(pg_get_serial_sequence('users', 'id'), 1, false);

INSERT INTO users(email, password, admin) VALUES ('email12@email.com', 'password', false, false);
INSERT INTO users(email, password, admin) VALUES ('email21@email.com', 'password', true, false);
