-- Your SQL goes here
CREATE TABLE IF NOT EXISTS sales
(
	id SERIAL PRIMARY KEY,
	seller_id INTEGER NOT NULL,
   	product_id INTEGER NOT NULL,
   	buyer_id INTEGER NOT NULL,
   	sale_date DATE NOT NULL DEFAULT CURRENT_DATE,
   	quantity INTEGER NOT NULL,
   	price INTEGER NOT NULL
);

INSERT INTO sales (seller_id, product_id, buyer_id, sale_date, quantity, price) VALUES ('1', '1', '1', '2019-01-21', '2', '2000');
INSERT INTO sales (seller_id, product_id, buyer_id, sale_date, quantity, price) VALUES ('1', '2', '2', '2019-02-17', '1', '800');
INSERT INTO sales (seller_id, product_id, buyer_id, sale_date, quantity, price) VALUES ('2', '2', '3', '2019-06-02', '1', '800');
INSERT INTO sales (seller_id, product_id, buyer_id, sale_date, quantity, price) VALUES ('3', '3', '4', '2019-05-13', '2', '2800');
