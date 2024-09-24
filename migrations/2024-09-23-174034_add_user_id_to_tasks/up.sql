-- Your SQL goes here
ALTER TABLE tasks
ADD COLUMN user_id UUID DEFAULT uuid_generate_v4() NOT NULL;
