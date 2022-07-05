-- Add up migration script here
CREATE TABLE IF NOT EXISTS users
(
    id          INTEGER PRIMARY KEY NOT NULL,
    username TEXT                NOT NULL,
    password TEXT                NOT NULL,
);