-- Your SQL goes here
CREATE TABLE IF NOT EXISTS answers (
  id SERIAL PRIMARY KEY,
  survies_id int REFERENCES survies(id),
  name text NOT NULL,
  value text NULL
)
