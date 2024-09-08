CREATE TABLE "tasks" (
    id SERIAL PRIMARY KEY,
    title TEXT,
    typ TEXT,
    priority TEXT,
    status TEXT,
    description TEXT,
    duration INTEGER,
    due_date BIGINT,
    project_id INTEGER,
    task_list TEXT[]
);

