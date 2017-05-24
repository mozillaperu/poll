-- Your SQL goes here
CREATE TABLE IF NOT EXISTS survies (
  id SERIAL PRIMARY KEY,
  title text NOT NULL,
  created date NOT NULL
)
