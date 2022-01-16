-- Add migration script here
PRAGMA foreign_keys= off;

-- create backup files table
ALTER TABLE files
    RENAME TO _files_old;

-- create backup hashes table
ALTER TABLE hashes
    RENAME TO _hashes_old;

-- create backup hash_tag_mappings table
ALTER TABLE hash_tag_mappings
    RENAME TO _hash_tag_mappings_old;

-- create backup hash_source_mappings table
ALTER TABLE hash_source_mappings
    RENAME TO _hash_source_mappings_old;

-- create content id table
CREATE TABLE content_descriptors
(
    id    INTEGER PRIMARY KEY AUTOINCREMENT,
    descriptor BLOB NOT NULL
);

CREATE UNIQUE INDEX content_descriptor_values ON content_descriptors (descriptor);

-- create content-id tag mappings table
CREATE TABLE cd_tag_mappings
(
    cd_id INTEGER NOT NULL REFERENCES content_descriptors (id),
    tag_id     INTEGER NOT NULL REFERENCES tags (id),
    PRIMARY KEY (cd_id, tag_id)
);

CREATE UNIQUE INDEX content_descriptor_tag_mapping_unique ON cd_tag_mappings (cd_id, tag_id);
CREATE INDEX content_descriptor_tag_mapping_tag ON cd_tag_mappings (tag_id);

-- create content-id source mappings table
CREATE TABLE cd_source_mappings
(
    cd_id INTEGER NOT NULL REFERENCES content_descriptors (id),
    source_id  INTEGER NOT NULL REFERENCES sources (id),
    PRIMARY KEY (cd_id, source_id)
);

CREATE UNIQUE INDEX content_descriptor_source_mapping_unique ON cd_source_mappings (cd_id, source_id);

-- create new files table
CREATE TABLE files
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    status     INTEGER      NOT NULL DEFAULT 10,
    storage_id INTEGER      NOT NULL REFERENCES storage_locations (id),
    cd_id INTEGER      NOT NULL REFERENCES content_descriptors (id),
    mime_type  VARCHAR(128) NOT NULL DEFAULT 'application/octet-stream'
);

CREATE INDEX files_contend_descriptor ON files (cd_id);

-- create metadata table
CREATE TABLE file_metadata
(
    file_id       INTEGER PRIMARY KEY REFERENCES files (id),
    size          INTEGER NOT NULL,
    name          VARCHAR(128),
    comment       VARCHAR(1024),
    import_time   DATETIME NOT NULL,
    creation_time DATETIME NOT NULL,
    change_time   DATETIME NOT NULL
);

CREATE UNIQUE INDEX file_metadata_file_id_unique ON file_metadata (file_id);

-- add content identifiers from hashes table
INSERT INTO content_descriptors
SELECT id, value
FROM _hashes_old;

-- add files from files table
INSERT INTO files (id, storage_id, cd_id, mime_type)
SELECT id, storage_id, hash_id AS content_id, mime_type
FROM _files_old;

-- add metadata from files table
INSERT INTO file_metadata
SELECT id AS file_id, size, name, comment, import_time, creation_time, change_time
FROM _files_old;

-- add content tag mappings
INSERT INTO cd_tag_mappings
SELECT hash_id AS content_id, tag_id
FROM _hash_tag_mappings_old;

-- add content id source mappings
INSERT INTO cd_source_mappings
SELECT hash_id AS content_id, source_id
FROM _hash_source_mappings_old;

-- drop all old tables
DROP TABLE _hash_source_mappings_old;
DROP TABLE _hash_tag_mappings_old;
DROP TABLE _files_old;
DROP TABLE _hashes_old;

pragma foreign_keys= on;