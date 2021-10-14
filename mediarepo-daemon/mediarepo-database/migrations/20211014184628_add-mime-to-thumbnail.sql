-- Add migration script here
ALTER TABLE thumbnails
    ADD COLUMN mime VARCHAR(128);