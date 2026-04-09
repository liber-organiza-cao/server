-- Add up migration script here
CREATE TABLE messages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    channel_id INTEGER NOT NULL,
    content TEXT NOT NULL,
    FOREIGN KEY (channel_id) REFERENCES channels (id) ON DELETE CASCADE
);