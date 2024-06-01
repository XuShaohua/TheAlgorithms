-- Your SQL goes here
CREATE TABLE IF NOT EXISTS product (
	product_id SERIAL PRIMARY KEY,
   	product_name VARCHAR(10)
);

INSERT INTO product (product_id, product_name) VALUES ('100', 'Nokia');
INSERT INTO product (product_id, product_name) VALUES ('200', 'Apple');
INSERT INTO product (product_id, product_name) VALUES ('300', 'Samsung');
