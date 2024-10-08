
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

ALTER TABLE tasks
    ALTER COLUMN id SET DATA TYPE UUID
    USING uuid_generate_v4(),
    ALTER COLUMN id SET DEFAULT uuid_generate_v4();
