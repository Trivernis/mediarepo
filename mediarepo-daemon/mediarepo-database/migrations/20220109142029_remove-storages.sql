-- Add migration script here
PRAGMA foreign_keys= off;

-- rename old files table
ALTER TABLE files
    RENAME TO _files_old;
-- rename metadata value (because of foreign key constraints)
ALTER TABLE file_metadata
    RENAME TO _file_metadata_old;

-- create new files table
CREATE TABLE files
(
    id        INTEGER PRIMARY KEY AUTOINCREMENT,
    status    INTEGER      NOT NULL DEFAULT 10,
    cd_id     INTEGER      NOT NULL REFERENCES content_descriptors (id),
    mime_type VARCHAR(128) NOT NULL DEFAULT 'application/octet-stream'
);
-- add data from files table
INSERT INTO files
SELECT id, status, cd_id, mime_type
FROM _files_old;

-- create metadata table
CREATE TABLE file_metadata
(
    file_id       INTEGER PRIMARY KEY REFERENCES files (id),
    size          INTEGER  NOT NULL,
    name          VARCHAR(128),
    comment       VARCHAR(1024),
    import_time   DATETIME NOT NULL,
    creation_time DATETIME NOT NULL,
    change_time   DATETIME NOT NULL
);

-- add back the old values
INSERT INTO file_metadata
SELECT *
FROM _file_metadata_old;

-- drop old tables
DROP TABLE _file_metadata_old;
DROP TABLE _files_old;
DROP TABLE storage_locations;

-- create indices on new tables
CREATE UNIQUE INDEX file_metadata_file_id_unique ON file_metadata (file_id);
CREATE INDEX files_content_descriptor ON files (cd_id);

PRAGMA foreign_keys= on;