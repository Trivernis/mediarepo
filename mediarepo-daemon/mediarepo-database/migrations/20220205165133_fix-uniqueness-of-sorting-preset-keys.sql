PRAGMA foreign_keys= off;
ALTER TABLE sorting_preset_keys
    RENAME TO _sorting_preset_keys_old;

CREATE TABLE sorting_preset_keys
(
    preset_id INTEGER REFERENCES sorting_presets (id) ON DELETE CASCADE,
    key_id    INTEGER REFERENCES sort_keys (id),
    key_index INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (preset_id, key_id, key_index)
);

INSERT INTO sorting_preset_keys SELECT * FROM _sorting_preset_keys_old;

DROP TABLE _sorting_preset_keys_old;

PRAGMA foreign_keys= on;
