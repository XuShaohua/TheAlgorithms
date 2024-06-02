-- Your SQL goes here
CREATE TABLE IF NOT EXISTS department (
	id INTEGER NOT NULL,
   	revenue INTEGER NOT NULL,
   	month VARCHAR(5) NOT NULL,
	PRIMARY KEY (id, month)
);

INSERT INTO department (id, revenue, month) VALUES ('1', '8000', 'Jan');
INSERT INTO department (id, revenue, month) VALUES ('2', '9000', 'Jan');
INSERT INTO department (id, revenue, month) VALUES ('3', '10000', 'Feb');
INSERT INTO department (id, revenue, month) VALUES ('1', '7000', 'Feb');
INSERT INTO department (id, revenue, month) VALUES ('1', '6000', 'Mar');
