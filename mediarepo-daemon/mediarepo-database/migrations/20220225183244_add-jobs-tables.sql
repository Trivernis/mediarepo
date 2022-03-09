CREATE TABLE jobs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    job_type INTEGER NOT NULL,
    name VARCHAR(255),
    next_run DATETIME,
    interval INTEGER
);

CREATE TABLE job_states (
    job_id INTEGER,
    key VARCHAR(128) NOT NULL DEFAULT 'default',
    value BLOB,
    PRIMARY KEY (job_id, key),
    FOREIGN KEY (job_id) REFERENCES jobs (id)
);