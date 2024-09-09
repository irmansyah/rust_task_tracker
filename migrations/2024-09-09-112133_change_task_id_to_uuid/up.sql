-- First, add the `uuid-ossp` extension if needed (optional)
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Alter the `id` column to `UUID`
ALTER TABLE tasks
    ALTER COLUMN id SET DATA TYPE UUID
    USING gen_random_uuid();

-- Ensure `id` is non-nullable and use `gen_random_uuid()` for new rows
ALTER TABLE tasks
    ALTER COLUMN id SET DEFAULT gen_random_uuid();
