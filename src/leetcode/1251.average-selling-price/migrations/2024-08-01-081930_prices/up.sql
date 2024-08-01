-- Your SQL goes here
CREATE TABLE IF NOT EXISTS prices (
	id SERIAL PRIMARY KEY,
	product_id int NOT NULL,
	start_date date NOT NULL,
	end_date date NOT NULL,
	price int NOT NULL,
	UNIQUE(product_id, start_date, end_date)
);

INSERT INTO prices (product_id, start_date, end_date, price) VALUES ('1', '2019-02-17', '2019-02-28', '5');
INSERT INTO prices (product_id, start_date, end_date, price) VALUES ('1', '2019-03-01', '2019-03-22', '20');
INSERT INTO prices (product_id, start_date, end_date, price) VALUES ('2', '2019-02-01', '2019-02-20', '15');
INSERT INTO prices (product_id, start_date, end_date, price) VALUES ('2', '2019-02-21', '2019-03-31', '30');
