-- Your SQL goes here

CREATE TABLE IF NOT EXISTS person
(
	id SERIAL PRIMARY KEY,
	email varchar(255) NOT NULL
);

INSERT INTO person (id, email) VALUES (1, 'john@example.com');
INSERT INTO person (id, email) VALUES (2, 'bob@example.com');
INSERT INTO person (id, email) VALUES (3, 'john@example.com');
