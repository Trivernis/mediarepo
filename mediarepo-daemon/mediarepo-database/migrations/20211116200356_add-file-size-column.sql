-- Add migration script here
ALTER TABLE files
    ADD COLUMN size INTEGER;