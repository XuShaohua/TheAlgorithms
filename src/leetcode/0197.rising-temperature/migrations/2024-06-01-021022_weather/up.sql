-- Your SQL goes here
CREATE TABLE IF NOT EXISTS weather (
	id SERIAL PRIMARY KEY,
	recordDate DATE DEFAULT CURRENT_DATE NOT NULL,
	temperature INTEGER NOT NULL
);

INSERT INTO weather (id, recordDate, temperature) VALUES (1, '2015-01-01', 10);
INSERT INTO weather (id, recordDate, temperature) VALUES (2, '2015-01-02', 25);
INSERT INTO weather (id, recordDate, temperature) VALUES (3, '2015-01-03', 20);
INSERT INTO weather (id, recordDate, temperature) VALUES (4, '2015-01-04', 30);
