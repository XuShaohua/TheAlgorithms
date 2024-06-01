-- Your SQL goes here
CREATE TABLE IF NOT EXISTS customers
(
	id SERIAL PRIMARY KEY,
	name VARCHAR(255) NOT NULL
);

INSERT INTO customers (id, name) VALUES (1, 'Joe');
INSERT INTO customers (id, name) VALUES (2, 'Henry');
INSERT INTO customers (id, name) VALUES (3, 'Sam');
INSERT INTO customers (id, name) VALUES (4, 'Max');
