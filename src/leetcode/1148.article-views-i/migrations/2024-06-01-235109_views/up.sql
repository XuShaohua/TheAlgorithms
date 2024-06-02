-- Your SQL goes here
CREATE TABLE IF NOT EXISTS views (
	id SERIAL PRIMARY KEY,
	article_id INTEGER NOT NULL,
   	author_id INTEGER NOT NULL,
   	viewer_id INTEGER NOT NULL,
   	view_date DATE NOT NULL DEFAULT CURRENT_DATE
);

INSERT INTO views (article_id, author_id, viewer_id, view_date) VALUES ('1', '3', '5', '2019-08-01');
INSERT INTO views (article_id, author_id, viewer_id, view_date) VALUES ('1', '3', '6', '2019-08-02');
INSERT INTO views (article_id, author_id, viewer_id, view_date) VALUES ('2', '7', '7', '2019-08-01');
INSERT INTO views (article_id, author_id, viewer_id, view_date) VALUES ('2', '7', '6', '2019-08-02');
INSERT INTO views (article_id, author_id, viewer_id, view_date) VALUES ('4', '7', '1', '2019-07-22');
INSERT INTO views (article_id, author_id, viewer_id, view_date) VALUES ('3', '4', '4', '2019-07-21');
INSERT INTO views (article_id, author_id, viewer_id, view_date) VALUES ('3', '4', '4', '2019-07-21');
