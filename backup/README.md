# filebase-backup

A lightweight automated backup service for the Filebase SQLite database.

---

## Overview

`filebase-backup` runs as a companion container alongside the Filebase server. It takes periodic, non-blocking database snapshots using SQLite and automatically purges old backup archives based on a configurable retention policy.

---

## How It Works

1. **Non-Blocking Snapshots**: Executes `VACUUM INTO` against the live SQLite database (`filebase.db`). Because WAL mode is active, this takes a full snapshot without interrupting active reads or writes.
2. **Atomic Writes**: Creates a temporary snapshot (`filebase-<timestamp>.db.tmp`) and renames it once complete to prevent corrupted or partial backups.

---

## Configuration

The service is configured via environment variables:

| Variable               | Description                                            | Default (`compose.yml`) |
| ---------------------- | ------------------------------------------------------ | ----------------------- |
| `INTERVAL_SECONDS`     | Time in seconds between consecutive backups            | `3600` (1 hour)         |
| `AGE_DAYS`             | Maximum age in days before old backup snapshots are purged | `7` days            |
| `TIMEOUT_MILLISECONDS` | SQLite busy timeout to acquire snapshot lock           | `10000` (10s)           |

---

## Usage

To build the image standalone:

```bash
cd backup
docker build -t filebase-backup .
```

---

## Restoring a Backup

To restore Filebase from a backup snapshot:

1. Stop the running containers:

   ```bash
   docker compose down
   ```

2. Replace the active database with the chosen backup file:

   ```bash
   cp ./.backups/filebase-<timestamp>.db ./.data/filebase.db
   ```

3. Restart the services:

   ```bash
   docker compose up -d
   ```
