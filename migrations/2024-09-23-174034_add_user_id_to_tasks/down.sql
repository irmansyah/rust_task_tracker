-- This file should undo anything in `up.sql`
ALTER TABLE tasks
DROP COLUMN user_id;
