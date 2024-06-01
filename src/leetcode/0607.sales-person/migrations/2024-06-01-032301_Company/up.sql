-- Your SQL goes here
CREATE TABLE IF NOT EXISTS Company
(
	com_id INTEGER PRIMARY KEY,
	name VARCHAR(255) NOT NULL,
	city VARCHAR(255)
);

INSERT INTO Company (com_id, name, city) VALUES ('1', 'RED', 'Boston');
INSERT INTO Company (com_id, name, city) VALUES ('2', 'ORANGE', 'New York');
INSERT INTO Company (com_id, name, city) VALUES ('3', 'YELLOW', 'Boston');
INSERT INTO Company (com_id, name, city) VALUES ('4', 'GREEN', 'Austin');
