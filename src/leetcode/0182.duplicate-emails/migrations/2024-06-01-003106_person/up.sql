-- Your SQL goes here

CREATE TABLE IF NOT EXISTS person
(
	id SERIAL PRIMARY KEY,
	email VARCHAR(255) NOT NULL
);

INSERT INTO person
(id, email)
VALUES
(1, 'a@b.com'),
(2, 'c@d.com'),
(3, 'a@b.com');
