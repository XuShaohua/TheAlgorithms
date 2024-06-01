-- Your SQL goes here
CREATE TABLE IF NOT EXISTS customer
(
	id SERIAL PRIMARY KEY,
	name VARCHAR(255) NOT NULL,
	referee_id INTEGER
);

insert into customer (id, name, referee_id) VALUES (1, 'Will', NULL);
insert into customer (id, name, referee_id) VALUES (2, 'Jane', NULL);
insert into customer (id, name, referee_id) VALUES (3, 'Alex', 2);
insert into customer (id, name, referee_id) VALUES (4, 'Bill', NULL);
insert into customer (id, name, referee_id) VALUES (5, 'Zack', 1);
insert into customer (id, name, referee_id) VALUES (6, 'Mark', 2);
