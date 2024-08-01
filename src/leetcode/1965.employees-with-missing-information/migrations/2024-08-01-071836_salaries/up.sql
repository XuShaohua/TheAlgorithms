-- Your SQL goes here
CREATE TABLE IF NOT EXISTS salaries (
	id SERIAL PRIMARY KEY,
	employee_id int UNIQUE NOT NULL,
   	salary int NOT NULL
);

INSERT INTO Salaries (employee_id, salary) VALUES ('5', '76071');
INSERT INTO Salaries (employee_id, salary) VALUES ('1', '22517');
INSERT INTO Salaries (employee_id, salary) VALUES ('4', '63539');
