-- Your SQL goes here
CREATE TABLE IF NOT EXISTS triangle
(
	x INTEGER NOT NULL,
	y INTEGER NOT NULL,
	z INTEGER NOT NULL,
	PRIMARY KEY (x, y, z)
);

INSERT INTO triangle (x, y, z) VALUES ('13', '15', '30');
INSERT INTO triangle (x, y, z) VALUES ('10', '20', '15');
