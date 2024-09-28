
CREATE TABLE "tasks" (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID DEFAULT uuid_generate_v4() NOT NULL,
    title TEXT,
    typ TEXT,
    priority TEXT,
    status TEXT,
    description TEXT,
    duration INTEGER,
    due_date BIGINT,
    project_id INTEGER,
    task_list TEXT[],
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

