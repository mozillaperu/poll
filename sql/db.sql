-- createdb poll
-- \list
-- \c poll
-- \dt

CREATE TABLE IF NOT EXISTS survies (
  id SERIAL PRIMARY KEY,
  title text NOT NULL,
  created date NOT NULL
)

CREATE TABLE IF NOT EXISTS answers (
  id SERIAL PRIMARY KEY,
  survies_id int REFERENCES survies(id),
  name text NOT NULL,
  value text NULL
)
