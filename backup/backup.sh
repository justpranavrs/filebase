#!/bin/sh
# Copyright 2026 Pranav R S.
# Use of this source code is governed by the MIT license that can be found in the LICENSE file.


set -eu

# Backup Configuration Variables
DB="${DB_PATH:-'/var/lib/filebase/data/filebase.db'}"
OUT_DIR="${DB_BACKUP_PATH:-'/var/lib/filebase/backups'}"
AGE="${AGE_DAYS:-7}"
INTERVAL="${INTERVAL_SECONDS:-21600}"
TIMEOUT="${TIMEOUT_MILLISECONDS:-10000}"

# Main Entrypoint for the Backup Loop at INTERVAL seconds.
while true; do
    TS="$(date +%F-%H%M%S)"
    TMP="$OUT_DIR/filebase-$TS.db.tmp"
    FINAL="$OUT_DIR/filebase-$TS.db"

    # Safely backup the active SQLite database
    sqlite3 -cmd ".timeout '$TIMEOUT'" "$DB" "VACUUM INTO '$TMP'"
    mv "$TMP" "$FINAL"
    echo "backup written: $FINAL"

    # Clean up backup files older than AGE days.
    del_count=$(find "$OUT_DIR" -name 'filebase-*.db*' -mtime +"$AGE" -print -delete| wc -l)
    echo "backup purge: deleted $del_count backup files older than $AGE days"
    sleep "$INTERVAL"
done
