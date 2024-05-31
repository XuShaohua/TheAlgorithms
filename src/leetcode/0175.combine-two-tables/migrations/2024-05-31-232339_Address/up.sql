-- Your SQL goes here

CREATE TABLE IF NOT EXISTS Address
(
	addressId SERIAL PRIMARY KEY,
	personId INTEGER NOT NULL,
	city varchar(64),
	state varchar(64)
);

-- Add records

INSERT INTO Address
(addressId, personId, city, state)
VALUES
(1, 2, 'New York City', 'New York'),
(2, 3, 'Leetcode', 'California');

