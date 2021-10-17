-- Add migration script here
PRAGMA foreign_keys=off;

ALTER TABLE tags RENAME TO _tags_old;
CREATE TABLE tags
(
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    namespace_id INTEGER,
    name         VARCHAR(128),
    FOREIGN KEY (namespace_id) REFERENCES namespaces (id)
);
CREATE UNIQUE INDEX tag_namespace_name_index ON tags (namespace_id, name);

INSERT INTO tags SELECT * FROM _tags_old;

DROP TABLE _tags_old;

ALTER TABLE hash_tag_mappings RENAME TO _hash_tag_mappings_old;
CREATE TABLE hash_tag_mappings
(
    hash_id INTEGER NOT NULL,
    tag_id  INTEGER NOT NULL,
    PRIMARY KEY (hash_id, tag_id),
    FOREIGN KEY (hash_id) REFERENCES hashes (id),
    FOREIGN KEY (tag_id) REFERENCES tags (id)
);
CREATE UNIQUE INDEX hash_tag_mappings_hash_tag ON hash_tag_mappings (hash_id, tag_id);

INSERT INTO hash_tag_mappings SELECT * FROM _hash_tag_mappings_old;

DROP TABLE _hash_tag_mappings_old;

PRAGMA foreign_keys=on;
