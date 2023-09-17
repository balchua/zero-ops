-- Add migration script here
CREATE TABLE IF NOT EXISTS platform
(
    id          INTEGER PRIMARY KEY NOT NULL,
    name        VARCHAR(250)        NOT NULL
);