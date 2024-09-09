-- Reverse the UUID change
ALTER TABLE tasks
    ALTER COLUMN id SET DATA TYPE INT
    USING (id::integer);
