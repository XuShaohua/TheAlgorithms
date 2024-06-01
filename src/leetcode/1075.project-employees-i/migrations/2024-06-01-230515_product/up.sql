-- Your SQL goes here
CREATE TABLE IF NOT EXISTS project (
	project_id INTEGER NOT NULL,
   	employee_id INTEGER NOT NULL,
	PRIMARY KEY (project_id, employee_id)
);

INSERT INTO project (project_id, employee_id) VALUES ('1', '1');
INSERT INTO project (project_id, employee_id) VALUES ('1', '2');
INSERT INTO project (project_id, employee_id) VALUES ('1', '3');
INSERT INTO project (project_id, employee_id) VALUES ('2', '1');
INSERT INTO project (project_id, employee_id) VALUES ('2', '4');
