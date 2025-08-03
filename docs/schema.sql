-- Clippoo Clipboard History Database Schema

CREATE TABLE IF NOT EXISTS clipboard_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    content TEXT NOT NULL UNIQUE,
    timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_default BOOLEAN NOT NULL DEFAULT FALSE
);

-- Index for efficient timestamp-based queries
CREATE INDEX IF NOT EXISTS idx_timestamp ON clipboard_history(timestamp DESC);

-- Index for finding the default entry quickly
CREATE INDEX IF NOT EXISTS idx_default ON clipboard_history(is_default);

-- Sample queries:

-- Get the most recent 10 entries
-- SELECT * FROM clipboard_history ORDER BY timestamp DESC LIMIT 10;

-- Get the current default entry
-- SELECT * FROM clipboard_history WHERE is_default = TRUE LIMIT 1;

-- Add a new entry (first clear default flag)
-- UPDATE clipboard_history SET is_default = FALSE WHERE is_default = TRUE;
-- INSERT INTO clipboard_history (content, is_default) VALUES ('new content', TRUE);

-- Clean up old entries (keep only last 50)
-- DELETE FROM clipboard_history 
-- WHERE id NOT IN (
--     SELECT id FROM clipboard_history 
--     ORDER BY timestamp DESC 
--     LIMIT 50
-- );