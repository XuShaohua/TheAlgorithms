-- Your SQL goes here
CREATE TABLE IF NOT EXISTS employees (
	id SERIAL PRIMARY KEY,
	employee_id int UNIQUE NOT NULL,
	name varchar(30) NOT NULL
);

INSERT INTO employees (employee_id, name) VALUES ('2', 'Crew');
INSERT INTO employees (employee_id, name) VALUES ('4', 'Haven');
INSERT INTO employees (employee_id, name) VALUES ('5', 'Kristian');
