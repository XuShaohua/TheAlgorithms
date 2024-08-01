-- Your SQL goes here
CREATE TABLE IF NOT EXISTS employees (
	id SERIAL PRIMARY KEY,
	employee_id int UNIQUE NOT NULL,
   	name varchar(20) NOT NULL,
	manager_id int,
	salary int NOT NULL
);

INSERT INTO employees (employee_id, name, manager_id, salary) VALUES ('3', 'Mila', '9', '60301');
INSERT INTO employees (employee_id, name, manager_id, salary) VALUES ('12', 'Antonella', NULL, '31000');
INSERT INTO employees (employee_id, name, manager_id, salary) VALUES ('13', 'Emery', NULL, '67084');
INSERT INTO employees (employee_id, name, manager_id, salary) VALUES ('1', 'Kalel', '11', '21241');
INSERT INTO employees (employee_id, name, manager_id, salary) VALUES ('9', 'Mikaela', NULL, '50937');
INSERT INTO employees (employee_id, name, manager_id, salary) VALUES ('11', 'Joziah', '6', '28485');
