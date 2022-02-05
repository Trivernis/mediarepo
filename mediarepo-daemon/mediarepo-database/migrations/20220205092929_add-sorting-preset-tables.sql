-- Add migration script here
CREATE TABLE sorting_presets (
    id INTEGER PRIMARY KEY AUTOINCREMENT
);

CREATE TABLE sort_keys (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    key_type INTEGER NOT NULL DEFAULT 0,
    ascending INTEGER NOT NULL CHECK (ascending IN (0, 1)),
    value VARCHAR(128)
);

CREATE TABLE sorting_preset_keys (
    preset_id INTEGER REFERENCES sorting_presets (id) ON DELETE CASCADE,
    key_id INTEGER REFERENCES sort_keys (id),
    key_index INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (preset_id, key_id)
);

CREATE INDEX sorting_preset_index ON sorting_preset_keys (preset_id);