-- Your SQL goes here

CREATE TABLE IF NOT EXISTS activity
(
	player_id INTEGER NOT NULL,
	device_id INTEGER NOT NULL,
	event_date DATE NOT NULL DEFAULT CURRENT_DATE,
	games_played INTEGER NOT NULL,
	PRIMARY KEY (player_id, event_date)
);

INSERT INTO activity (player_id, device_id, event_date, games_played) VALUES ('1', '2', '2016-03-01', '5');
INSERT INTO activity (player_id, device_id, event_date, games_played) VALUES ('1', '2', '2016-05-02', '6');
INSERT INTO activity (player_id, device_id, event_date, games_played) VALUES ('2', '3', '2017-06-25', '1');
INSERT INTO activity (player_id, device_id, event_date, games_played) VALUES ('3', '1', '2016-03-02', '0');
INSERT INTO activity (player_id, device_id, event_date, games_played) VALUES ('3', '4', '2018-07-03', '5');
