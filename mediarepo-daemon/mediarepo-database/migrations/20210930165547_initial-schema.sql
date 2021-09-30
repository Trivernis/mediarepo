CREATE TABLE storage_locations
(
    id   INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(128) UNIQUE NOT NULL,
    path VARCHAR(255)        NOT NULL
);


CREATE TABLE hashes
(
    id    INTEGER PRIMARY KEY AUTOINCREMENT,
    value TEXT NOT NULL
);

CREATE UNIQUE INDEX hashes_value_index ON hashes (value);


CREATE TABLE files
(
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    type          INTEGER  NOT NULL DEFAULT 0,
    name          VARCHAR(128),
    comment       VARCHAR(1024),
    storage_id    INTEGER  NOT NULL,
    hash_id       INTEGER  NOT NULL,
    import_time   DATETIME NOT NULL,
    creation_time DATETIME NOT NULL,
    change_time   DATETIME NOT NULL,
    FOREIGN KEY (storage_id) REFERENCES storage_locations (id),
    FOREIGN KEY (hash_id) REFERENCES hashes (id)
);


CREATE TABLE namespaces
(
    id   INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(128) UNIQUE NOT NULL
);

CREATE UNIQUE INDEX namespaces_name_index ON namespaces (name);


CREATE TABLE tags
(
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    namespace_id INTEGER,
    name         VARCHAR(128) UNIQUE NOT NULL,
    FOREIGN KEY (namespace_id) REFERENCES namespaces (id)
);

CREATE UNIQUE INDEX tag_name_index ON tags (name);



CREATE TABLE sources
(
    id  INTEGER PRIMARY KEY AUTOINCREMENT,
    url VARCHAR(512) NOT NULL
);

CREATE UNIQUE INDEX sources_value_index ON sources (url);


CREATE TABLE hash_tag_mappings
(
    hash_id INTEGER NOT NULL,
    tag_id  INTEGER NOT NULL,
    PRIMARY KEY (hash_id, tag_id),
    FOREIGN KEY (hash_id) REFERENCES hashes (id),
    FOREIGN KEY (tag_id) REFERENCES tags (id)
);


CREATE TABLE hash_source_mappings
(
    hash_id   INTEGER NOT NULL,
    source_id INTEGER NOT NULL,
    PRIMARY KEY (hash_id, source_id),
    FOREIGN KEY (hash_id) REFERENCES hashes (id),
    FOREIGN KEY (source_id) REFERENCES sources (id)
)