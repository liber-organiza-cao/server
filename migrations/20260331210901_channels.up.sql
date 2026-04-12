-- Add up migration script here

CREATE TABLE channels (
    id BLOB PRIMARY KEY NOT NULL,
    name TEXT NOT NULL
);