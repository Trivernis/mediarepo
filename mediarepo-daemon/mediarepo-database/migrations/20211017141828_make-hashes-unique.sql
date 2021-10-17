-- Add migration script here
DELETE FROM thumbnails WHERE file_id NOT IN (SELECT MIN(files.id) FROM files GROUP BY hash_id);
DELETE FROM files WHERE ROWID NOT IN (SELECT MIN(ROWID) FROM files GROUP BY hash_id);
DELETE FROM thumbnails WHERE hash_id NOT IN (SELECT MIN(hashes.id) FROM hashes GROUP BY value);
DELETE FROM files WHERE hash_id NOT IN (SELECT MIN(hashes.id) FROM hashes GROUP BY value);
DELETE FROM hashes WHERE ROWID NOT IN (SELECT MIN(ROWID) FROM hashes GROUP BY value);
CREATE UNIQUE INDEX hash_value_index ON hashes (value);
CREATE UNIQUE INDEX file_hash_id ON files (hash_id);