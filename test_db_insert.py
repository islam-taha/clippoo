#!/usr/bin/env python3
import sqlite3
from datetime import datetime

# Connect to the database
conn = sqlite3.connect('/home/islam/.local/share/clippoo/clipboard.db')
cursor = conn.cursor()

# Insert test entries
test_entries = [
    f"Test clipboard entry {i}" for i in range(1, 61)
]

for i, entry in enumerate(test_entries):
    try:
        cursor.execute(
            "INSERT INTO clipboard_history (content, is_default) VALUES (?, ?)",
            (entry, i == 0)  # First entry is default
        )
    except sqlite3.IntegrityError:
        # Entry already exists, update timestamp
        cursor.execute(
            "UPDATE clipboard_history SET timestamp = CURRENT_TIMESTAMP WHERE content = ?",
            (entry,)
        )

conn.commit()

# Show current entries
cursor.execute("SELECT COUNT(*) FROM clipboard_history")
print(f"Total entries: {cursor.fetchone()[0]}")

cursor.execute("SELECT id, substr(content, 1, 30) as preview, datetime(timestamp) FROM clipboard_history ORDER BY timestamp DESC LIMIT 10")
print("\nLatest 10 entries:")
for row in cursor.fetchall():
    print(f"ID: {row[0]:3} | {row[1]:30} | {row[2]}")

conn.close()