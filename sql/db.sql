-- createdb poll
-- \list
-- \c poll
-- \dt

CREATE TABLE IF NOT EXISTS survey(
  id SERIAL PRIMARY KEY,
  title text NOT NULL,
  created date NOT NULL
);

CREATE TABLE IF NOT EXISTS answer(
  id SERIAL PRIMARY KEY,
  survey_id int REFERENCES survey(id),
  name text NOT NULL,
  value text NULL
);
