# Quick reference

- **Maintained by**: [Pranav R S](https://github.com/justpranavrs)
- **Where to get help**: [GitHub Discussions](https://github.com/justpranavrs/filebase/discussions) or [GitHub Issues](https://github.com/justpranavrs/filebase/issues)

## Supported tags and respective `Dockerfile` links

- [`0.1.0-alpha`](https://github.com/justpranavrs/filebase/blob/cdd7082a7783546426d327cf9c0e554d1899cc18/Dockerfile)

## Quick reference (cont.)

- **Where to file issues**: [https://github.com/justpranavrs/filebase/issues](https://github.com/justpranavrs/filebase/issues)
- **Supported architectures**: `linux/amd64`
- **Source of this description**: [repo's docker/ directory](https://github.com/justpranavrs/filebase/tree/main/docker)

# What is Filebase?

<p>
  <img src="https://raw.githubusercontent.com/justpranavrs/filebase/main/static/favicon.svg" width="64" height="64" alt="Filebase">
</p>

Filebase is an open-source, self-hosted document management system built for storing personal records, certificates, and files locally. It operates with zero cloud dependencies, using SQLite for structured metadata and local disk storage for document files.

The web interface is server-rendered using Askama templates and HTMX, providing fast in-page updates without client-side single-page framework overhead.

# How to use this image

### Start a Filebase instance

```console
$ mkdir -p data storage
$ docker run --name some-filebase \
    -p 3000:3000 \
    --user "$(id -u):$(id -g)" \
    -v ./data:/var/lib/filebase/data \
    -v ./storage:/var/lib/filebase/storage \
    -d justpranavrs/filebase:0.1.0-alpha
```

Database migrations execute automatically when the container starts. Once running, access the web interface at `http://localhost:3000`.

### ... via `docker compose`

Example `compose.yml` for Filebase:

```yaml
services:
  filebase:
    image: justpranavrs/filebase:0.1.0-alpha
    container_name: filebase
    restart: unless-stopped
    ports:
      - "3000:3000"
    volumes:
      - ./data:/var/lib/filebase/data
      - ./storage:/var/lib/filebase/storage
```

Run `docker compose up -d`, wait for initialization to complete, and visit `http://localhost:3000`.

## Where to store data

Two directories inside the container require persistent storage:

- `/var/lib/filebase/data`: Stores the active SQLite database file (`filebase.db`).
- `/var/lib/filebase/storage`: Stores uploaded document payloads.

Important Note: Mount host directories or named volumes to these locations. If these paths are not mounted, all stored files and database records will be lost whenever the container is updated or removed.

### Arbitrary `--user` Notes

This image runs as the non-root user `nonroot` with UID `65532` by default. When mounting host directories on Linux, ensure the directories exist and are writable by UID `65532` (for example via `chmod 777 data storage`), or pass `--user "$(id -u):$(id -g)"` to run the container using your host user credentials.

## Environment Variables

The Filebase image uses environment variables to configure its runtime behavior. All variables have sensible defaults pre-configured in the container.

### `SERVER_HOST`

This variable defines the network interface Filebase binds to inside the container. Defaults to `0.0.0.0`.

### `SERVER_PORT`

This variable defines the port Filebase listens on inside the container. Defaults to `3000`.

### `DATABASE_URL`

This variable defines the SQLite database connection string. Defaults to `sqlite:/var/lib/filebase/data/filebase.db`.

### `STORAGE_DIR`

This variable defines the directory where uploaded documents are stored. Defaults to `/var/lib/filebase/storage`.

### `LOG_LEVEL`

This variable controls logging verbosity (`debug`, `info`, `warn`, `error`). Defaults to `info`.

### `SERVER_TIMEOUT_SECONDS`

This optional variable sets the HTTP request timeout in seconds. Defaults to `30`.

### `DATABASE_MAX_CONNECTIONS`

This optional variable sets the maximum number of SQLite connections in the pool. Defaults to `10`.

### `DATABASE_TIMEOUT_SECONDS`

This optional variable sets the database lock acquire timeout in seconds. Defaults to `15`.

## Image details

- **Base image**: `gcr.io/distroless/static-debian13:nonroot`
- **User**: `65532:65532` (`nonroot`)
- **Binary**: Statically compiled Rust binary linked against `musl`

# License

View [license information](https://github.com/justpranavrs/filebase/blob/main/LICENSE) for the software contained in this image.
