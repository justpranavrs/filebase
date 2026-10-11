# Quick reference

- **Maintained by**: [Pranav R S](https://github.com/justpranavrs)
- **Where to get help**: [GitHub Discussions](https://github.com/justpranavrs/filebase/discussions) or [GitHub Issues](https://github.com/justpranavrs/filebase/issues)

## Supported tags and respective `Dockerfile` links

- [`0.1.0-alpha`](https://github.com/justpranavrs/filebase/blob/cdd7082a7783546426d327cf9c0e554d1899cc18/backup/Dockerfile)

## Quick reference (cont.)

- **Where to file issues**: [https://github.com/justpranavrs/filebase/issues](https://github.com/justpranavrs/filebase/issues)
- **Supported architectures**: `linux/amd64`
- **Source of this description**: [repo's backup/ directory](https://github.com/justpranavrs/filebase/tree/main/backup)

# What is filebase-backup?

<p>
  <img src="https://raw.githubusercontent.com/justpranavrs/filebase/main/static/favicon.svg" width="64" height="64" alt="Filebase">
</p>

`filebase-backup` is an automated companion backup container for the Filebase SQLite database. It runs alongside the core Filebase service, periodically taking non-blocking database snapshots using SQLite `VACUUM INTO` and purging old backup files according to a retention policy.

Because Filebase operates SQLite with Write-Ahead Logging (WAL) enabled, snapshots complete without acquiring write locks or interrupting active application traffic.

# How to use this image

### ... via `docker compose`

`filebase-backup` runs alongside Filebase as a companion container. A complete `compose.yml` example:

```yaml
services:
  filebase:
    image: justpranavrs/filebase:0.1.0-alpha
    container_name: filebase-server
    restart: unless-stopped
    ports:
      - "3000:3000"
    volumes:
      - ./data:/var/lib/filebase/data
      - ./storage:/var/lib/filebase/storage

  backups:
    image: justpranavrs/filebase-backup:0.1.0-alpha
    container_name: filebase-backup
    restart: unless-stopped
    environment:
      - INTERVAL_SECONDS=3600
      - AGE_DAYS=7
      - TIMEOUT_MILLISECONDS=10000
    volumes:
      - ./data:/var/lib/filebase/data
      - ./backups:/var/lib/filebase/backups
```

Start the services:

```console
$ docker compose up -d
```

### Standalone `docker run`

To run the backup container standalone against an existing Filebase data directory:

```console
$ docker run -d \
    --name filebase-backup \
    -v /my/own/data:/var/lib/filebase/data \
    -v /my/own/backups:/var/lib/filebase/backups \
    -e INTERVAL_SECONDS=3600 \
    -e AGE_DAYS=7 \
    --restart unless-stopped \
    justpranavrs/filebase-backup:0.1.0-alpha
```

## Where to store data

The backup container mounts two volumes:

- `/var/lib/filebase/data`: Must point to the same directory used by the Filebase server where `filebase.db` resides.
- `/var/lib/filebase/backups`: Destination directory on the host where timestamped backup snapshots (`filebase-<timestamp>.db`) are written and retained.

### Arbitrary `--user` Notes

The container executes as the non-root user `65532:65532`. Ensure that host directories mounted into `/var/lib/filebase/backups` and `/var/lib/filebase/data` have read and write permissions for UID `65532` (or are world-writable).

## Environment Variables

The backup container is configured using the following environment variables:

### `INTERVAL_SECONDS`

Time in seconds between consecutive database backup snapshots. Defaults to `21600` (6 hours) in the script, and `3600` (1 hour) in `compose.yml`.

### `AGE_DAYS`

Maximum age in days before older backup snapshots are automatically purged from `/var/lib/filebase/backups`. Defaults to `7`.

### `TIMEOUT_MILLISECONDS`

SQLite busy timeout in milliseconds to wait before failing if a database lock cannot be acquired. Defaults to `10000` (10 seconds).

## Restoring a Backup

To restore Filebase from a backup snapshot:

1. Stop the running containers:

```console
$ docker compose down
```

2. Replace the active database with the chosen snapshot:

```console
$ cp ./backups/filebase-<timestamp>.db ./data/filebase.db
```

3. Restart the containers:

```console
$ docker compose up -d
```

## Image details

- **Base image**: `alpine:3.24`
- **User**: `65532:65532`
- **Packages**: `sqlite`

# License

View [license information](https://github.com/justpranavrs/filebase/blob/main/LICENSE) for the software contained in this image.
