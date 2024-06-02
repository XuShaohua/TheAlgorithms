
CREATE TABLE IF NOT EXISTS employee (
	id SERIAL PRIMARY KEY,
   	salary INTEGER NOT NULL
);

INSERT INTO employee (id, salary) VALUES ('1', '100');
INSERT INTO employee (id, salary) VALUES ('2', '200');
INSERT INTO employee (id, salary) VALUES ('3', '300');
