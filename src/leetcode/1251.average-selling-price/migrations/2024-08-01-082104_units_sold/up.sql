-- Your SQL goes here
CREATE TABLE IF NOT EXISTS UnitsSold (
	id SERIAL PRIMARY KEY,
	product_id int NOT NULL,
   	purchase_date date NOT NULL,
   	units int NOT NULL
);

INSERT INTO UnitsSold (product_id, purchase_date, units) VALUES ('1', '2019-02-25', '100');
INSERT INTO UnitsSold (product_id, purchase_date, units) VALUES ('1', '2019-03-01', '15');
INSERT INTO UnitsSold (product_id, purchase_date, units) VALUES ('2', '2019-02-10', '200');
INSERT INTO UnitsSold (product_id, purchase_date, units) VALUES ('2', '2019-03-22', '30');
