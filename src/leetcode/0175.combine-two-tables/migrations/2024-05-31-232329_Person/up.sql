-- Your SQL goes here

CREATE TABLE IF NOT EXISTS Person 
(
	personId SERIAL PRIMARY KEY,
	lastName varChar(32) NOT NULL,
	firstName varChar(32) NOT NULL
);

-- Add records

INSERT INTO Person
(personId, lastName, firstName)
VALUES
(1, 'Wang', 'Allen'),
(2, 'Alice', 'Bob');
