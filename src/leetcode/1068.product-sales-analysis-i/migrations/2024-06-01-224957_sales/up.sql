-- Your SQL goes here
CREATE TABLE IF NOT EXISTS sales (
	sale_id INTEGER NOT NULL, 
	product_id INTEGER NOT NULL, 
	year INTEGER NOT NULL, 
	quantity INTEGER NOT NULL, 
	price INTEGER NOT NULL,
	PRIMARY KEY(sale_id, year)
);

INSERT INTO sales (sale_id, product_id, year, quantity, price) VALUES ('1', '100', '2008', '10', '5000');
INSERT INTO sales (sale_id, product_id, year, quantity, price) VALUES ('2', '100', '2009', '12', '5000');
INSERT INTO sales (sale_id, product_id, year, quantity, price) VALUES ('7', '200', '2011', '15', '9000');
