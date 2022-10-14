CREATE TABLE worklog (
  id SERIAL PRIMARY KEY NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT now(),
  description VARCHAR NOT NULL
);

CREATE TABLE tags (
  id SERIAL PRIMARY KEY NOT NULL,
  name VARCHAR NOT NULL
);

CREATE TABLE tag_keywords (
  id SERIAL PRIMARY KEY NOT NULL,
  keyword VARCHAR NOT NULL, 
  tag_id SERIAL NOT NULL,
  FOREIGN KEY (tag_id) REFERENCES tags(id)
);

CREATE TABLE work_tags (
  work_id SERIAL NOT NULL,
  tag_id SERIAL NOT NULL,
  FOREIGN KEY(work_id) REFERENCES worklog(id),
  FOREIGN KEY(tag_id) REFERENCES tags(id),
  PRIMARY KEY(work_id, tag_id)
);
