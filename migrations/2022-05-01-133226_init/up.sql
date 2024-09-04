CREATE TABLE "tasks" (
  id SERIAL PRIMARY KEY,
  typ VARCHAR NOT NULL,
  title VARCHAR NOT NULL,
  description VARCHAR NOT NULL,
  status VARCHAR NOT NULL
  duration INTEGER,
  due_date BIGINT,
  task_list TEXT[],
);
