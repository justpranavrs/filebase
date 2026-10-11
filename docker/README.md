# Docker

Run Filebase using the official pre-built Docker container image.

---

## Overview

Filebase is an open-source, self-hosted document management system for personal files and records. It runs with zero cloud dependencies, using SQLite for structured metadata and local disk storage for files.

---

## Pull the Image

Pull the latest official image from Docker Hub:

```bash
docker pull justpranavrs/filebase:latest
```

---

## Quick Start

### Using `docker run`

Run the container with persistent storage volumes for SQLite metadata and uploaded files:

```bash
docker run -d \
  --name filebase \
  -p 3000:3000 \
  -v ./data:/var/lib/filebase/data \
  -v ./storage:/var/lib/filebase/storage \
  --restart unless-stopped \
  justpranavrs/filebase:latest
```

Once running, access the web interface at `http://localhost:3000`. Database migrations execute automatically on container startup.

---

## Container Volumes & Ports

| Type | Path / Port | Description |
| :--- | :--- | :--- |
| **Port** | `3000` | HTTP web server port |
| **Volume** | `/var/lib/filebase/data` | Directory storing the SQLite database (`filebase.db`) |
| **Volume** | `/var/lib/filebase/storage` | Directory storing uploaded document files |

---

## Configuration

The image includes sensible defaults and can be configured with environment variables:

| Variable | Default | Description |
| :--- | :--- | :--- |
| `SERVER_HOST` | `0.0.0.0` | Network interface to bind to |
| `SERVER_PORT` | `3000` | Port Filebase listens on inside the container |
| `DATABASE_URL` | `sqlite:/var/lib/filebase/data/filebase.db` | SQLite connection URL |
| `STORAGE_DIR` | `/var/lib/filebase/storage` | Path where uploaded files are stored |
| `LOG_LEVEL` | `info` | Logging verbosity (`debug`, `info`, `warn`, `error`) |
