-- Your SQL goes here
CREATE TABLE IF NOT EXISTS employee
(
	empId SERIAL PRIMARY KEY,
	name VARCHAR(32),
	supervisor INTEGER,
	salary INTEGER
);

-- Add records

INSERT INTO employee
(empId, name, supervisor, salary)
VALUES
(3, 'Brad', NULL, 4000),
(1, 'John', 3, 1000),
(2, 'Dan', 3, 2000),
(4, 'Thomas', 3, 4000);
