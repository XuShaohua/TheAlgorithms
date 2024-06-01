-- Your SQL goes here
CREATE TABLE IF NOT EXISTS orders
(
	id SERIAL PRIMARY KEY,
	customerId INTEGER NOT NULL
);

INSERT INTO orders (id, customerId) VALUES (1, 3);
INSERT INTO orders (id, customerId) VALUES (2, 1);
