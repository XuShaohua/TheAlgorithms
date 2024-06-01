-- Your SQL goes here
CREATE TABLE IF NOT EXISTS ActorDirector
(
	actor_id INTEGER NOT NULL,
	director_id INTEGER NOT NULL,
	timestamp INTEGER PRIMARY KEY
);

INSERT INTO ActorDirector (actor_id, director_id, timestamp) VALUES ('1', '1', '0');
INSERT INTO ActorDirector (actor_id, director_id, timestamp) VALUES ('1', '1', '1');
INSERT INTO ActorDirector (actor_id, director_id, timestamp) VALUES ('1', '1', '2');
INSERT INTO ActorDirector (actor_id, director_id, timestamp) VALUES ('1', '2', '3');
INSERT INTO ActorDirector (actor_id, director_id, timestamp) VALUES ('1', '2', '4');
INSERT INTO ActorDirector (actor_id, director_id, timestamp) VALUES ('2', '1', '5');
INSERT INTO ActorDirector (actor_id, director_id, timestamp) VALUES ('2', '1', '6');
