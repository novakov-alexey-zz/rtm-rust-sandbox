CREATE TABLE tasks (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  added TIMESTAMP with time zone NOT NULL,
  due TIMESTAMP with time zone NOT NULL,
  list VARCHAR NOT NULL,
  notes TEXT NOT NULL,
  completed BOOLEAN NOT NULL DEFAULT 'f',
  priority TEXT NOT NULL
);