-- Your SQL goes here
CREATE TABLE IF NOT EXISTS Orders
(
	order_id INTEGER PRIMARY KEY,
	order_date DATE NOT NULL DEFAULT CURRENT_DATE,
	com_id INTEGER NOT NULL,
	sales_id INTEGER NOT NULL,
	amount INTEGER NOT NULL
);

INSERT INTO Orders (order_id, order_date, com_id, sales_id, amount) VALUES ('1', '1/1/2014', '3', '4', '10000');
INSERT INTO Orders (order_id, order_date, com_id, sales_id, amount) VALUES ('2', '2/1/2014', '4', '5', '5000');
INSERT INTO Orders (order_id, order_date, com_id, sales_id, amount) VALUES ('3', '3/1/2014', '1', '1', '50000');
INSERT INTO Orders (order_id, order_date, com_id, sales_id, amount) VALUES ('4', '4/1/2014', '1', '4', '25000');
