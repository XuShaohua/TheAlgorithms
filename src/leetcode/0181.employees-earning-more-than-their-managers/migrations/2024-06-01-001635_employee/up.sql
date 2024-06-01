-- Your SQL goes here

CREATE TABLE IF NOT EXISTS employee
(
	id SERIAL PRIMARY KEY,
	name VARCHAR(32),
	salary INTEGER,
	managerId INTEGER
);


-- Add records
INSERT INTO employee
(id, name, salary, managerId)
VALUES
(1, 'Joe', 70000  , 3),
(2, 'Henry', 80000, 4),
(3, 'Sam', 60000, Null),
(4, 'Max', 90000, Null);
