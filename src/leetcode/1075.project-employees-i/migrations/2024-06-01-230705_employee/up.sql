-- Your SQL goes here
CREATE TABLE IF NOT EXISTS employee
(
	employee_id SERIAL PRIMARY KEY,
   	name VARCHAR(10),
   	experience_years INTEGER NOT NULL
);

INSERT INTO employee (employee_id, name, experience_years) VALUES ('1', 'Khaled', '3');
INSERT INTO employee (employee_id, name, experience_years) VALUES ('2', 'Ali', '2');
INSERT INTO employee (employee_id, name, experience_years) VALUES ('3', 'John', '1');
INSERT INTO employee (employee_id, name, experience_years) VALUES ('4', 'Doe', '2');
