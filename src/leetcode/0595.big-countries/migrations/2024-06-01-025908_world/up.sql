-- Your SQL goes here
CREATE TABLE IF NOT EXISTS world
(
	id SERIAL PRIMARY KEY,
	name VARCHAR(255) NOT NULL,
	continent VARCHAR(255),
	area INTEGER,
	population INTEGER,
	gdp BIGINT
);

INSERT INTO world (name, continent, area, population, gdp) VALUES ('Afghanistan', 'Asia', '652230', '25500100', '20343000000');
INSERT INTO world (name, continent, area, population, gdp) VALUES ('Albania', 'Europe', '28748', '2831741', '12960000000');
INSERT INTO world (name, continent, area, population, gdp) VALUES ('Algeria', 'Africa', '2381741', '37100000', '188681000000');
INSERT INTO world (name, continent, area, population, gdp) VALUES ('Andorra', 'Europe', '468', '78115', '3712000000');
INSERT INTO world (name, continent, area, population, gdp) VALUES ('Angola', 'Africa', '1246700', '20609294', '100990000000');
