-- Add migration script here
CREATE TABLE thumbnails (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    hash_id INTEGER UNIQUE NOT NULL,
    storage_id INTEGER NOT NULL,
    file_id INTEGER NOT NULL,
    height INTEGER NOT NULL,
    width INTEGER NOT NULL,
    FOREIGN KEY (hash_id) REFERENCES hashes (id),
    FOREIGN KEY (storage_id) REFERENCES storage_locations (id),
    FOREIGN KEY (file_id) REFERENCES files (id)
);

CREATE UNIQUE INDEX thumbnail_file_resolution ON thumbnails (file_id, height, width);