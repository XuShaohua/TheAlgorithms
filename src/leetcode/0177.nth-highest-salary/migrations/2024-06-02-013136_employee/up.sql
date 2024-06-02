-- Your SQL goes here
CREATE TABLE IF NOT EXISTS employee (
	id SERIAL PRIMARY KEY,
   	salary INTEGER NOT NULL
);

INSERT INTO Employee (id, salary) VALUES ('1', '100');
INSERT INTO Employee (id, salary) VALUES ('2', '200');
INSERT INTO Employee (id, salary) VALUES ('3', '300');
