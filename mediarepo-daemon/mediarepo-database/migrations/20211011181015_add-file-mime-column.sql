-- Add migration script here
ALTER TABLE files
    ADD COLUMN mime_type VARCHAR(128);