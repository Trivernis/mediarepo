-- Add migration script here
CREATE INDEX index_hash_tag_mappings_tag_id ON hash_tag_mappings (tag_id);
CREATE INDEX index_tag_name ON tags (name);
