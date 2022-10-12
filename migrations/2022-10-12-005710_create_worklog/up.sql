CREATE TABLE worklog (
  id INTEGER PRIMARY KEY,
  created_at DEFAULT CURRENT_TIMESTAMP,
  description VARCHAR NOT NULL
);

CREATE TABLE tags (
  id INTEGER PRIMARY KEY,
  name VARCHAR
);

CREATE TABLE tag_keywords (
  id INTEGER PRIMARY KEY,
  keyword VARCHAR,
  tag_id INTEGER,
  FOREIGN KEY (tag_id) REFERENCES tags(id)
);

CREATE TABLE work_tags (
  work_id INTEGER,
  tag_id INTEGER,
  FOREIGN KEY(work_id) REFERENCES worklog(id),
  FOREIGN KEY(tag_id) REFERENCES tags(id),
  PRIMARY KEY(work_id, tag_id)
);
