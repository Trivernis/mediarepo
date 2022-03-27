CREATE TABLE job_states (
    job_type INTEGER NOT NULL,
    value BLOB,
    PRIMARY KEY (job_type)
);