-- Your SQL goes here

CREATE TABLE IF NOT EXISTS teacher (
	id SERIAL PRIMARY KEY,
	teacher_id int NOT NULL,
   	subject_id int NOT NULL,
   	dept_id int NOT NULL
);

CREATE UNIQUE INDEX teacher_subj_dep ON teacher (
	subject_id,
	dept_id
);

insert into teacher (teacher_id, subject_id, dept_id) VALUES ('1', '2', '3');
insert into teacher (teacher_id, subject_id, dept_id) VALUES ('1', '2', '4');
insert into teacher (teacher_id, subject_id, dept_id) VALUES ('1', '3', '3');
insert into teacher (teacher_id, subject_id, dept_id) VALUES ('2', '1', '1');
insert into teacher (teacher_id, subject_id, dept_id) VALUES ('2', '2', '1');
insert into teacher (teacher_id, subject_id, dept_id) VALUES ('2', '3', '1');
insert into teacher (teacher_id, subject_id, dept_id) VALUES ('2', '4', '1');
