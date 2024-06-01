-- Your SQL goes here
CREATE TABLE IF NOT EXISTS SalesPerson
(
	sales_id INTEGER PRIMARY KEY,
	name VARCHAR(255) NOT NULL,
	salary INTEGER NOT NULL,
	commission_rate INTEGER,
	hire_date DATE
);

INSERT INTO SalesPerson (sales_id, name, salary, commission_rate, hire_date) VALUES ('1', 'John', '100000', '6', '4/1/2006');
INSERT INTO SalesPerson (sales_id, name, salary, commission_rate, hire_date) VALUES ('2', 'Amy', '12000', '5', '5/1/2010');
INSERT INTO SalesPerson (sales_id, name, salary, commission_rate, hire_date) VALUES ('3', 'Mark', '65000', '12', '12/25/2008');
INSERT INTO SalesPerson (sales_id, name, salary, commission_rate, hire_date) VALUES ('4', 'Pam', '25000', '25', '1/1/2005');
INSERT INTO SalesPerson (sales_id, name, salary, commission_rate, hire_date) VALUES ('5', 'Alex', '5000', '10', '2/3/2007');
