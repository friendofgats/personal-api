-- Your SQL goes here
CREATE TABLE information (
  id SERIAL PRIMARY KEY,
  body TEXT NOT NULL,
  section TEXT NOT NULL,
  verbosity  INT NOT NULL
)