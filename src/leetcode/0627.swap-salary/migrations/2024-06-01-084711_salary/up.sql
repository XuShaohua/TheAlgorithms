-- Your SQL goes here
CREATE TABLE IF NOT EXISTS salary
(
	id SERIAL PRIMARY KEY,
	name VARCHAR(100) NOT NULL,
	sex char(1) NOT NULL,
	salary INTEGER NOT NULL
);

INSERT INTO salary (id, name, sex, salary) VALUES ('1', 'A', 'm', '2500');
INSERT INTO salary (id, name, sex, salary) VALUES ('2', 'B', 'f', '1500');
INSERT INTO salary (id, name, sex, salary) VALUES ('3', 'C', 'm', '5500');
INSERT INTO salary (id, name, sex, salary) VALUES ('4', 'D', 'f', '500');
