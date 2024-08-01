-- Your SQL goes here
CREATE TABLE IF NOT EXISTS employees (
	id SERIAL PRIMARY KEY,
	employee_id int NOT NULL,
	name varchar(30) NOT NULL,
	salary int NOT NULL
);

INSERT INTO employees (employee_id, name, salary) VALUES ('2', 'Meir', '3000');
INSERT INTO employees (employee_id, name, salary) VALUES ('3', 'Michael', '3800');
INSERT INTO employees (employee_id, name, salary) VALUES ('7', 'Addilyn', '7400');
INSERT INTO employees (employee_id, name, salary) VALUES ('8', 'Juan', '6100');
INSERT INTO employees (employee_id, name, salary) VALUES ('9', 'Kannon', '7700');
