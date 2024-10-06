-- Add migration script here
CREATE TABLE IF NOT EXISTS todo (
    id serial NOT NULL PRIMARY KEY,
    task text NOT NULL UNIQUE,
    is_done BOOLEAN DEFAULT false NOT NULL, 
    created_at TIMESTAMP(0) WITH TIME ZONE NOT NULL DEFAULT NOW(),
    author_id INT NOT NULL REFERENCES users ON DELETE CASCADE
);