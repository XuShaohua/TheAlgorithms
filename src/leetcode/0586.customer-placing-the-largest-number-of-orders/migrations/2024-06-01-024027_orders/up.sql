-- Your SQL goes here
CREATE TABLE IF NOT EXISTS orders
(
	order_number SERIAL PRIMARY KEY,
	customer_number INTEGER NOT NULL
);

INSERT INTO orders (order_number, customer_number) VALUES (1, 1);
INSERT INTO orders (order_number, customer_number) VALUES (2, 2);
INSERT INTO orders (order_number, customer_number) VALUES (3, 3);
INSERT INTO orders (order_number, customer_number) VALUES (4, 3);
