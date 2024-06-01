-- Your SQL goes here
CREATE TABLE IF NOT EXISTS bonus
(
	id SERIAL PRIMARY KEY,
	empId INTEGER,
	bonus INTEGER
);

-- Add records
INSERT INTO bonus
(empId, bonus)
VALUES
(2, 500),
(4, 2000);
