-- Your SQL goes here
CREATE TABLE IF NOT EXISTS product
(
	product_id SERIAL PRIMARY KEY,
   	product_name VARCHAR(10) NOT NULL,
   	unit_price INTEGER NOT NULL
);

INSERT INTO product (product_id, product_name, unit_price) VALUES ('1', 'S8', '1000');
INSERT INTO product (product_id, product_name, unit_price) VALUES ('2', 'G4', '800');
INSERT INTO product (product_id, product_name, unit_price) VALUES ('3', 'iPhone', '1400');
