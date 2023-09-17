-- Add migration script here
CREATE TABLE IF NOT EXISTS events
(
    id              INTEGER PRIMARY KEY NOT NULL,
    name            VARCHAR(20)        NOT NULL,
    active          BOOLEAN             NOT NULL DEFAULT 0,
    created_date    DATETIME          NOT NULL DEFAULT CURRENT_TIMESTAMP,
    platform_id     INTEGER             NOT NULL,
    FOREIGN KEY (platform_id) REFERENCES platform(id)
);